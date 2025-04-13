use std::collections::HashMap;
use macroquad::math::Vec2;

use crate::neuron::Neuron;

const GRID_SIZE: f32 = 50.0;

#[derive(Debug)]
struct GridCell {
    total_position: Vec2,
    count: usize,
}

fn build_spatial_grid(neurons: &HashMap<u32, Neuron>) -> HashMap<(i32, i32), GridCell> {
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