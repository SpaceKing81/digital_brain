pub mod grid {
    use std::collections::HashMap;
    use macroquad::math::Vec2;
    
    use dashmap::DashMap;
    use rayon::prelude::*;

    use crate::neuron::Neuron;
    use crate::consts::*;

    #[derive(Debug)]
    pub struct GridCell {
        total_position: Vec2,
        count: f32,
    }
    impl GridCell {
        pub fn build_spatial_grid(neurons: &HashMap<u32, Neuron>) -> HashMap<(i32,i32), GridCell> {
            // Concurrently accumulate into a DashMap
            let grid = DashMap::new();
        
            neurons.par_iter().for_each(|(_, neuron)| {
                let pos = neuron.position;
                let key = (
                    (pos.x / GRID_SIZE).floor() as i32,
                    (pos.y / GRID_SIZE).floor() as i32,
                );
        
                // DashMap lets you do this without Mutex around the whole map
                grid.entry(key)
                    .and_modify(|cell: &mut GridCell| {
                        cell.total_position += pos;
                        cell.count += 1.0;
                    })
                    .or_insert_with(|| GridCell {
                        total_position: pos,
                        count: 1.0,
                    });
            });
        
            // Convert back to a regular HashMap
            grid.into_iter().collect()
        }
    pub fn compute_repulsion_from_grid(
        position: Vec2,
        grid_key: (i32, i32),
        grid: &HashMap<(i32, i32), GridCell>,
    ) -> Vec2 {
        let mut force = Vec2::ZERO;

        for dx in -2..=2 {
            for dy in -2..=2 {
                let neighbor_key = (grid_key.0 + dx, grid_key.1 + dy);

                if let Some(cell) = grid.get(&neighbor_key) {
                    if cell.count.floor() == 0.0 {
                        continue;
                    }

                    let center:Vec2 = cell.total_position / cell.count;
                    let dir = position - center;
                    let distance = dir.length().max(ELECTRIC_SUFRACE);
                    let repulsion_strength = COULOMB / distance.powi(2);

                    force += (dir.normalize() * repulsion_strength) * cell.count;
                }
            }
        }

        force
    }

}
}

pub mod update_threads {

    use std::collections::HashMap;
    use macroquad::math::Vec2;
    use std::sync::{Arc, Mutex};
    use rayon::prelude::*;
    use crate::Neuron;
    use crate::Axion;
    use crate::grid::grid::GridCell;
    use crate::consts::*;


    #[derive(Debug)]
    pub struct NeuronUpdate {
    pub id: u32,
    pub new_position: Vec2,
    }
    #[derive(Debug)]
    pub struct AxionUpdate {
    pub id: u128,
    pub new_happyness: u32,
    }



    pub fn parallel_neuron_step<F, G>(
        neurons: &HashMap<u32, Neuron>,
        axions: &HashMap<u128, Axion>,
        grid: &HashMap<(i32, i32), GridCell>,
        center: Vec2,
        center_force_fn: F,
        spring_force_fn: G,
    ) -> (Vec<NeuronUpdate>, Vec<AxionUpdate>)
    where
        F: Fn(u32, Vec2) -> Option<Vec2> + Sync + Send,
        G: Fn(u32, u32) -> Option<Vec2> + Sync + Send,
        {

            
            let neurons_snapshot: Vec<(u32, Neuron)> = neurons.iter().map(|(&id, n)| (id, n.clone())).collect();
            let axions_snapshot = axions;
            // 1) Parallel-map each neuron → (its NeuronUpdate, Vec<AxionUpdate>)
            let all_updates: Vec<(NeuronUpdate, Vec<AxionUpdate>)> =
                neurons_snapshot
                .into_par_iter()
                .map(|(id, neuron)| {
                    let mut total_force = Vec2::ZERO;
                    // center force
                    total_force -= center_force_fn(id, center).unwrap_or_default();
                    // repulsion
                    let key = (
                        (neuron.position.x / GRID_SIZE).floor() as i32,
                        (neuron.position.y / GRID_SIZE).floor() as i32,
                    );
                    total_force += GridCell::compute_repulsion_from_grid(
                        neuron.position, key, grid
                    );

                    // collect axion updates locally
                    let mut local_axions = Vec::new();
                    for ax_id in &neuron.input_axions {
                        if let Some(ax) = axions_snapshot.get(ax_id) {
                            total_force -= spring_force_fn(id, ax.id_source).unwrap_or_default();
                            local_axions.push(AxionUpdate {
                                id: *ax_id,
                                new_happyness: neuron.happyness,
                            });
                        }
                    }
                    for ax_id in &neuron.output_axions {
                        if let Some(ax) = axions_snapshot.get(ax_id) {
                            total_force -= spring_force_fn(id, ax.id_sink).unwrap_or_default();
                        }
                    }

                    let neuron_update = NeuronUpdate {
                        id,
                        new_position: neuron.position + total_force,
                    };
                    (neuron_update, local_axions)
                })
                .collect();

        // 2) split into two Vecs
        let (neuron_updates, axion_lists): (Vec<NeuronUpdate>, Vec<Vec<AxionUpdate>>) =
        all_updates.into_iter().unzip();

        // 3) flatten the Vec<Vec<AxionUpdate>> into Vec<AxionUpdate>
        let axion_updates: Vec<AxionUpdate> =
            axion_lists.into_iter().flatten().collect();

        (neuron_updates, axion_updates)
    }
}