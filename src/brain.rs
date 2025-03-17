use std::collections::{HashMap, HashSet};
use macroquad::{
  color::*, math::Vec2, rand, shapes::*
  // window::{screen_width,screen_height},
};
use std::u128;

use crate::{Axion, Input, Neuron, Output};

const GRAVITY: f32 = 0.01;
const GRAVITY_SUFRACE: f32 = 150.0;
const ELECTRIC_SUFRACE: f32 = 11.0;
const COULOMB: f32 = 10000.0;  
const SPRING:f32 = 0.01;
const TIME_STEP: f32 = 0.05;

const FIRED_INPUT:Color = Color::new(1.0, 0.5, 0.0, 1.0);
const WAITING_INPUT:Color = Color::new(0.5, 0.25, 0.0, 1.0);
const FIRED_OUTPUT:Color = Color::new(0.0, 0.25, 0.5, 1.0);
const WAITING_OUTPUT:Color = Color::new(0.0, 0.5, 1.0, 1.0);

pub struct Brain {
  clock:u128,

  pub neurons: HashMap<u32, Neuron>,
  pub axions: HashMap<u128,Axion>,
  pub inputs:HashMap<u32, Input>,
  pub outputs:HashMap<u32, Output>,

  num_of_neurons: u32,
  num_of_axions: u128,
  num_of_inputs: u32,
  num_of_outputs: u32,


  active_neurons:HashSet<u32>,
}

impl Brain {
  pub fn new() -> Brain {
    Brain {
      clock:0,

      neurons: HashMap::new(),
      axions: HashMap::new(),
      inputs:HashMap::new(),
      outputs:HashMap::new(),

      num_of_neurons:0,
      num_of_axions:0,
      num_of_inputs: 0,
      num_of_outputs: 0,


      active_neurons:HashSet::new(),
    }
  }
  pub fn spin_up_new(&mut self, num_neurons: u32, num_input:u32) {
    // for _ in 0..num_neurons {

    // }
    // set up neurons and axions
    // Step 1: Create neurons
      for _ in 0..num_neurons+10 {
        self.add_neuron();
      }
      self.num_of_neurons += num_neurons;
      // Step 2: Connect neurons with axons
      for _ in 0..(num_neurons * 2) {
        // Randomly select source and sink neurons
        let source_id = *self.neurons.keys().nth(rand::gen_range(0,self.neurons.len() as usize)).unwrap();
        let sink_id = *self.neurons.keys().nth(rand::gen_range(0,self.neurons.len() as usize)).unwrap();

        // Ensure source and sink are different
        if source_id != sink_id {
            self.add_axion(source_id, sink_id);
            self.num_of_axions += 1;
        }
      }
        let mut bad_connections = Vec::new();
        for i in 0..num_neurons {
          if let Some(neuron) = self.neurons.get(&i) {
            if neuron.output_axions.is_empty() {bad_connections.push(i);}
          }
        }
        for i in bad_connections {
          self.no_more_outputs(i);
        }
  

    // set up input neurons
    self.num_of_inputs = num_input;
    for id in 0..num_input {
      let mut input = Input::new(id);
      // connect in neurons
      for _ in 0..rand::gen_range(1,std::cmp::max(self.num_of_neurons,2)) {
        let i = rand::gen_range(0,self.num_of_neurons);
        if self.neurons.contains_key(&i) && !input.neurons.contains(&i) {
          input.neurons.push(i);
        }
      }

      self.inputs.insert(id, input);
    }
  self.temp_for_output();
  }
  pub fn tick(&mut self, input:bool) {
    // one tick passes
    self.clock += 1;
    // if an input is triggered
    if input {
      for id in 0..self.num_of_inputs {
        self.active_neurons.extend(self.inputs[&id].neurons.clone());
      }
    }

    let active_neurons: Vec<u32> = self.active_neurons.drain().collect();
    let mut axions_to_remove = Vec::new();
    let mut neurons_to_remove= Vec::new();
    println!("{} neruons to fire this tick", active_neurons.len());
    for neuron_id in active_neurons {
      if let Some(neuron) = self.neurons.get_mut(&neuron_id) {
        // update the input neuron happyness
        let input_axions = neuron.input_axions.clone();
        let happyness = neuron.happyness;
        for axion_id in input_axions {
          if let Some(axion) = self.axions.get_mut(&axion_id) {
            axion.update_happyness(happyness);
          }
        }
        // update the neurons
        neuron.update(self.clock);
        // Check if it should die
        if neuron.check_to_kill() {neurons_to_remove.push(neuron_id) }
        // check if it should fire
        if neuron.ready_to_fire() {
          // get the outputs
          let delta_t = neuron.delta_t;
          let output_axions = neuron.output_axions.clone();
          neuron.fired();

          for axion_id in output_axions {
            if let Some(axion) = self.axions.get_mut(&axion_id) {
              let (input_id, strength) = axion.fire(delta_t);
              if strength != 0 {
                // update all the input neuron strenght memories
                if let Some(input_neuron) = self.neurons.get_mut(&input_id) {
                input_neuron.inputs.push(strength);
                // add them to the next active neuron cycle
                self.active_neurons.insert(input_id);
                }
              } else {axions_to_remove.push(axion_id);}
            }
          }
        }
        
        }}
          // remove all inactive neurons
    for axion_id in axions_to_remove {self.remove_axion(axion_id);}
    for neuron_id in neurons_to_remove {self.no_more_outputs(neuron_id);}

  }
  pub fn no_more_outputs(&mut self, neuron_id: u32) {
    if let Some(neuron) = self.neurons.get(&neuron_id) {
      let roll = rand::gen_range(0,70);

      if roll + neuron.happyness < 50 {
        // Create new connections
        for _ in 0..rand::gen_range(5,10) {
          let sink_id = *self.neurons.keys().nth(rand::gen_range(0,self.neurons.len())).unwrap();
          if sink_id != neuron_id {
            self.add_axion(neuron_id, sink_id);
          }
        }
      } else {
        // Commit suicide
        self.remove_neuron(neuron_id);
      }
    }
  }
  // Can be found and removed at the end of spin up new
  fn temp_for_output(&self) {
    if &self.num_of_outputs == &5 {
      return
    }
  }

}

/// Graphics
impl Brain {
  pub fn update_layout(&mut self, center:Vec2) {
    let neuron_ids: Vec<u32> = self.neurons.keys().cloned().collect();
    let mut new_positions: HashMap<u32, Vec2> = HashMap::new();

    // Spring Attraction
    for (_id,axion) in &self.axions {
      let source = axion.id_source;
      let sink = axion.id_sink;
      let pos1 = self.neurons.get(&source).unwrap().position.clone();
      let pos2 = self.neurons.get(&sink).unwrap().position.clone();
      let distance_s = pos1.distance(pos2);
      if distance_s > 0.0 { // Prevent division by zero
        let direction_s = (pos1 - pos2) / distance_s;
        let spring = SPRING * distance_s;
        let delta = spring * direction_s * TIME_STEP;
        new_positions.entry(source).and_modify(|p| *p -= delta).or_insert(-delta);
        new_positions.entry(sink).and_modify(|p| *p += delta).or_insert(delta);
      }
    }
    for &id1 in &neuron_ids {
      let mut delta = Vec2::ZERO;
      let pos1 = self.neurons[&id1].position;
      // Gravity Attraction
      let distance_g = pos1.distance(center);
      if distance_g > GRAVITY_SUFRACE { // Prevent division by zero
        let direction_g = (pos1 - center) / distance_g;
        let gravity = GRAVITY * distance_g;
        delta -= gravity * direction_g * TIME_STEP;
      }
      for &id2 in &neuron_ids {
        if id1 != id2 {
          let pos2 = self.neurons[&id2].position;
          // Like-Charge Repulsion
          let distance_e = pos1.distance(pos2);
          if distance_e > ELECTRIC_SUFRACE { // Prevent division by zero
            let direction_e = (pos1 - pos2) / distance_e;
            let electric = COULOMB / (distance_e * distance_e);
            delta += electric * direction_e * TIME_STEP;
          }
        }
      }
      if let Some(offset_spring) = new_positions.get(&id1) {
        self.neurons.entry(id1).and_modify(|p| p.position += offset_spring.clone() + delta);
      } else {
        self.neurons.entry(id1).and_modify(|p| p.position += delta);
      }
    }
  }
  pub fn draw(&self) {

    // Draw axons first (so they are behind neurons)
    for axion in self.axions.values() {
      self.draw_axion(axion);
    }
    // Draw neurons
    for neuron in self.neurons.values() {
      neuron.draw();
    }
    for input in self.inputs.values() {
      let color = if input.tick < 5 {FIRED_INPUT} else {WAITING_INPUT};
      draw_circle(input.position.x, input.position.y, 10.0, color);
      // Crimson
    }
    for output in self.outputs.values() {
      let color = if output.tick < 5 {FIRED_OUTPUT} else {WAITING_OUTPUT};
      draw_circle(output.position.x, output.position.y, 10.0, color); // Wintery blue
    }
}


  fn draw_axion(&self, axion:&Axion) {
    let (source_id, sink_id, color) = axion.get_to_draw();
      if let (Some(source), Some(sink)) = (
        self.neurons.get(&source_id),
        self.neurons.get(&sink_id),
      ) {
        draw_line(
          source.position.x,
          source.position.y,
          sink.position.x,
          sink.position.y,
          2.0,
          color,
        );
      }
  }
}





impl Brain {
  pub fn add_neuron(&mut self) -> u32 {
    self.num_of_neurons +=1;
    let id = self.neurons.keys().max().unwrap_or(&0) + 1; // Generate a unique ID
    self.neurons.insert(id, Neuron::new());
    id
  }
  pub fn add_axion(&mut self, source_id: u32, sink_id: u32) -> u128 {
    self.num_of_axions +=1;
    let id = self.axions.keys().max().unwrap_or(&0) + 1; // Generate a unique ID
    let axion = Axion::new(source_id, sink_id, id);
    self.axions.insert(id, axion);

    // Update neuron connections
    if let Some(source_neuron) = self.neurons.get_mut(&source_id) {
      source_neuron.output_axions.push(id);
    }
    if let Some(sink_neuron) = self.neurons.get_mut(&sink_id) {
      sink_neuron.input_axions.push(id);
    }

    id
  }

  pub fn remove_neuron(&mut self, neuron_id: u32) {
      if let Some(neuron) = self.neurons.remove(&neuron_id) {
          // Remove all input axons
          self.num_of_neurons -= 1;
          for axion_id in neuron.input_axions {
              self.remove_axion(axion_id);
          }
          // Remove all output axons
          for axion_id in neuron.output_axions {
              self.remove_axion(axion_id);
          }
      }
  }
  pub fn remove_axion(&mut self, axion_id: u128) {
    if let Some(axion) = self.axions.remove(&axion_id) {
      // Remove axon from source neuron's output list
      if let Some(source_neuron) = self.neurons.get_mut(&axion.id_source) {
        source_neuron.output_axions.retain(|&id| id != axion_id);
      }
      // Remove axon from sink neuron's input list
      if let Some(sink_neuron) = self.neurons.get_mut(&axion.id_sink) {
        sink_neuron.input_axions.retain(|&id| id != axion_id);
      }
    }
  }
}
// need a list for all the neurons
// need a list for all the axions

// need a count for all the neurons
// need a count for all the axions

// need a list of input-nodes
// need a list of output-nodes
// need special connection for the input nodes -> neurons
// need special connection for the neurons -> output nodes

// need a master clock
// need a list of all activated neurons