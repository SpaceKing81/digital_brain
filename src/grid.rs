use std::collections::HashMap;
use macroquad::math::Vec2;

use crate::neuron::Neuron;
const GRID_SIZE: f32 = 50.0;

#[derive(Debug)]
pub struct GridCell {
    total_position: Vec2,
    count: usize,
}
impl GridCell {
  pub fn build_spatial_grid(neurons: &HashMap<u32, Neuron>) -> HashMap<(i32, i32), Self> {
      let mut grid = HashMap::new();

      for neuron in neurons.values() {
          let pos = neuron.position;
          let key = (
              (pos.x / GRID_SIZE).floor() as i32,
              (pos.y / GRID_SIZE).floor() as i32,
          );

          let cell = grid.entry(key).or_insert(GridCell {
              total_position: Vec2::ZERO,
              count: 0,
          });

          cell.total_position += pos;
          cell.count += 1;
      }

      grid
  }
  pub fn compute_repulsion_from_grid(
    position: Vec2,
    grid_key: (i32, i32),
    grid: &HashMap<(i32, i32), Self>,
) -> Vec2 {
    let mut force = Vec2::ZERO;

    for dx in -1..=1 {
        for dy in -1..=1 {
            let neighbor_key = (grid_key.0 + dx, grid_key.1 + dy);

            if let Some(cell) = grid.get(&neighbor_key) {
                if cell.count == 0 {
                    continue;
                }

                let center = cell.total_position / cell.count as f32;
                let dir = position - center;
                let distance = dir.length().max(1.0); // Avoid divide-by-zero
                let repulsion_strength = 100.0 / distance.powi(2); // Tune this constant

                force += dir.normalize() * repulsion_strength * cell.count as f32;
            }
        }
    }

    force
}

}