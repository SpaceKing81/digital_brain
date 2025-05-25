use std::collections::{HashMap, HashSet};
// use rayon::prelude::*;
use macroquad::{
  // color::*, 
  math::Vec2, rand::{self, gen_range}, shapes::*, window::{screen_height, screen_width},
};

use crate::{
  //
  axon::Axon, neuron::Neuron, internal_consts::*, consts::*, grid::{grid::*, update_threads::*}
  //
};

#[derive(Clone, Debug, PartialEq)]
pub struct Spirion {
  pub clock:u128,

  neurons: HashMap<u32, Neuron>,
  axons: HashMap<u128,Axon>,
  output_ids: HashSet<u32>,
  input_ids: HashSet<u128>,

  num_of_neurons: u32,
  num_of_axons: u128,

  active_neurons:HashSet<u32>,
}
// Public
impl Spirion {
  /// New brain with specifed inputs and outputs, optional number of Neurons.
  /// Default is 500 Neurons
  pub fn spin_up_new(num_neurons: Option<u32>, num_input: u128, num_output: u32) -> (Self, Vec<u128>, Vec<u32>) {
    let num_neurons = num_neurons.unwrap_or(500);
    // Step 0: Create Brain
    let mut brain = Self::new();
    
    // Step 1: Add outputs  
    for _ in 0..num_output {
      brain.add_output();
    }
    let output_ids: HashSet<u32> = brain.output_ids.clone();

    // Step 2: Create neurons
    for _ in 0..(num_neurons + 10) {
      brain.add_neuron();
    }

    // Cache neuron IDs into a Vec for efficient random access
    let neuron_ids: Vec<u32> = brain.neurons.keys().copied().collect();
    let len = neuron_ids.len();

    // Step 2: Connect neurons with axons
    for _ in 0..(num_neurons * 2) {
      let source_id = neuron_ids[rand::gen_range(1, len)];
      let sink_id = neuron_ids[rand::gen_range(1, len)];
      
      if output_ids.contains(&source_id) && !output_ids.contains(&sink_id) {
        brain.add_axon(sink_id,source_id);
        brain.num_of_axons += 1;
      } else if source_id != sink_id {
        brain.add_axon(source_id, sink_id);
        brain.num_of_axons += 1;
      }
    }

    // Identify and mark neurons without outputs
    for id in &neuron_ids[..num_neurons as usize] {
      if let Some(neuron) = brain.neurons.get(id) {
        if neuron.output_axons.is_empty() && !neuron.is_output {
          brain.no_more_outputs(*id);
        }
      }
    }

    // Step 3: Add + Configure Inputs
    let mut input_ids = Vec::new();
    while brain.input_ids.len() < num_input as usize {
      let sink_id = &neuron_ids[rand::gen_range(1, neuron_ids.len())];
      if !output_ids.contains(sink_id) && brain.neurons.contains_key(sink_id) {
        input_ids.push(brain.add_input(*sink_id));
      }
    }

    let output_ids = output_ids.iter().copied().collect();

    println!("Brain initialized. Thinking...");
    (brain, input_ids, output_ids)
  }
  
  /// Ticks over the brain simulation for however many specified ticks, with a default of 1 iteration.
  /// No input, however all the output id's are spat out as an Option<Vec>
  pub fn tick(&mut self, num_iterations:Option<u32>) -> Option<Vec<u32>> {
    let mut output = Vec::new();
    if num_iterations == Some(0) {panic!("Need a number larger then 0 iterations")}
    // Cannot render more at once with no input then a single standard_deviation without a cascading upshooting of happyness values!!???
    for _ in 0..(std::cmp::min(num_iterations.unwrap_or(1),(ONE_STANDARD_DEV_THRESHOLD - 1).abs() as u32)) {
      // one tick passes
      self.clock += 1;
      
      let active_neurons_to_iter: Vec<u32> = self.active_neurons.drain().collect();
      let active_neurons: HashSet<u32> = active_neurons_to_iter.iter().copied().collect();
      
      let mut axons_to_remove = Vec::new();
      let mut neurons_to_remove= Vec::new();

      // same but the first is the sink neuron, and the second is the source neurons
      let mut axons_to_add_sink: Vec<(u32, Vec<u32>)> = Vec::new();
      // Contains a connection pair, with the source neuron id followed by a vec of sink neurons
      let mut axons_to_add_source: Vec<(u32, Vec<u32>)> = Vec::new();

      let mut neurons_to_add:Vec<(usize,usize)> = Vec::new();

      for neuron_id in active_neurons_to_iter {
        let mut has_input = false;
        if let Some(neuron) = self.neurons.get_mut(&neuron_id) {
          // update the input neuron happyness
          let input_axons = neuron.input_axons.clone();
          let happyness = neuron.happyness;
          for axon_id in input_axons {
            // Checks to see if there are any inputs to this neuron
            if self.input_ids.contains(&axon_id) {has_input = true;}

            if let Some(axon) = self.axons.get_mut(&axon_id) {
              axon.update_happyness(happyness);
            }
          }
          // update the neurons
          let (num_in, num_out) = neuron.update(self.clock);

          // Find the requisate number of input-output additions
          if num_in.is_negative() {
            for _ in 0..num_in.abs() {
              let index = gen_range(0, neuron.input_axons.len());
              let id = neuron.input_axons[index];
              axons_to_remove.push(id);
            }
          } else {
            let mut sources = Vec::new();
            for _ in 0..num_in {
              let source = gen_range(1, self.num_of_neurons);
              sources.push(source);
            }
            axons_to_add_sink.push((neuron_id,sources));
          }
          if num_out.is_negative() {
            for _ in 0..num_out.abs() {
              let index = gen_range(0, neuron.output_axons.len());
              let id = neuron.output_axons[index];
              axons_to_remove.push(id);
            }
          } else {
            let mut sinks = Vec::new();
            for _ in 0..num_out {
              let sink = gen_range(1, self.num_of_neurons);
              sinks.push(sink);
            }
            axons_to_add_source.push((neuron_id,sinks));
          }

          // Check if it should die
          if neuron.check_to_kill(has_input) {neurons_to_remove.push(neuron_id)}

          // Check if it wants to spawn a new neuron 
          if neuron.want_to_reproduce() {
            neurons_to_add.push((neuron.input_axons.len(),neuron.output_axons.len()));
          }

          // Check if it should fire
          if neuron.ready_to_fire() {
            let delta_t = neuron.fired();
            let output_axons = neuron.output_axons.clone();
            // Check if its an output
            if neuron.is_output {output.push(neuron_id);continue;}
            for axon_id in output_axons {
              if let Some(axon) = self.axons.get_mut(&axon_id) {
                let (input_id, strength) = axon.fire_axon(delta_t);
                if strength != 0 {
                  // update all the input neuron strength memories
                  if let Some(input_neuron) = self.neurons.get_mut(&input_id) {
                    input_neuron.inputs.push(strength);
                  // add them to the next active neuron cycle if its not a repeat
                    if !active_neurons.contains(&input_id) {
                      self.active_neurons.insert(input_id);
                    }
                  }
                } else {axons_to_remove.push(axon_id);}
              }}}}}

      // Remove stuff
      for axon_id in axons_to_remove {self.remove_axon(axon_id);}
      for neuron_id in neurons_to_remove {self.no_more_outputs(neuron_id);}

      // TODO: add sink axons
      // TODO: add source axons
      
      for (inp,out) in neurons_to_add {
        self.spawn_new_neuron(inp, out);
      }

      todo!();
      

  }
  if output.is_empty() {
    return None;
  }
  Some(output)
  }
  
  /// Allows the addition of sensory input to the brain, takes a vec of (id, strength) pairs
  pub fn brain_input(&mut self, inputs:Option<Vec<(u128, i32)>>) {
    // Check if theres something in it
    if inputs.is_none() {return}

    for (input_id, strength) in inputs.unwrap() {
      // Check that it exists
      if !self.input_ids.contains(&input_id) {panic!("Invalid Input id passed in");}
  
      // Collect the neuron id
      if let Some(input) = self.axons.get_mut(&input_id) {
        let sink = input.id_sink;
        if let Some(neuron) = self.neurons.get_mut(&sink) {
          input.fire_axon(neuron.delta_t);
          neuron.inputs.push(strength);

          // Add to active neuron
          if !self.active_neurons.contains(&sink) {
            self.active_neurons.insert(sink);
          }

        } else { panic!("Library Error: Input axon not connected to a present neuron") }
      } else { panic!("Library Error: Input axon not in axon list") }
    }
  }

  /// Rewards Spirion with a level of intensity, ranging from 0 (None) to 10 (Endless Bliss)
  /// Makes it feel good, hopefully making the brain happy
  pub fn reward(&mut self, intensity:Option<u32>) {
    if intensity == None {return;}
    let intensity = std::cmp::min(intensity.unwrap(), 10);
    // collect a vec with all the tick frequencies
    // collect a vec with all the input ids
    let mut frequencies: Vec<u32> = Vec::new();
    let mut id: Vec<u128> = Vec::new();
    for i in &self.input_ids {
      if let Some(axon) = self.axons.get(i) {
        id.push(*i);
        frequencies.push(axon.avg_t);
      }
    }
    // loop a number of times based on intensity (maybe 5 * intensity?)
    for tick in 0..(intensity * ITERATION_MULTIPLIER) {
      let mut inputs:Vec<(u128,i32)> = Vec::new();
      for i in 0..frequencies.len() {
        if tick % frequencies[i] == 0 && frequencies[i] < tick {
          inputs.push((id[i], MAX_THRESHOLD));
        }
      }
      // tick once
      self.tick(None);
    }
  }
  /// Causes pain to Spirion by firing with a level of chaos based on the intensity.
  /// 0 (None) to 10 (Pure hellish agony) 
  pub fn pain(&mut self, intensity:Option<u32>) {
    if intensity == None {return;}
    let intensity = std::cmp::min(intensity.unwrap(), 10);

    for _ in 0..(intensity * ITERATION_MULTIPLIER) {
      let mut inputs:Vec<(u128,i32)> = Vec::new();
      for &id in &self.input_ids {
        if rand::gen_range(-2, 1) >= 0 {
          inputs.push((id, MAX_THRESHOLD));
        }
      }
      // tick once
      self.tick(None);
    }
    //  loop a number of times based on intensity (maybe 5 * intensity?)
    //  chose random inputs
    //  fire them with random strengths
    //  tick once
  }
}

// Mechanics
impl Spirion {
  fn new() -> Self {
    Spirion {
      clock:0,

      neurons: HashMap::new(),
      axons: HashMap::new(),
      output_ids: HashSet::new(),
      input_ids: HashSet::new(),

      num_of_neurons:0,
      num_of_axons:0,


      active_neurons:HashSet::new(),
    }
  }
  fn spring_force(&self, id1:u32, id2:u32) -> Option<Vec2> {
    if id1 != id2 {return None}
    let pos1 = self.neurons[&id1].pos;
    let pos2 = self.neurons[&id2].pos;
    let distance_s = pos1.distance(pos2);

    if distance_s > SPRING_NORMAL { 
      let direction_s = (pos1 - pos2) / distance_s;
      let spring = SPRING * distance_s;
      return Some(spring * direction_s * TIME_STEP);
    }
    None
  }
  fn center_force(&self, id1:u32, center:Vec2) -> Option<Vec2> {
    let pos1 = self.neurons[&id1].pos;
    let distance_g = pos1.distance(center);
    if distance_g > GRAVITY_SUFRACE { 
      let direction_g = (pos1 - center) / distance_g;
      let gravity = GRAVITY * distance_g;
      return Some(gravity * direction_g * TIME_STEP);
    }
  None
  }
}

/// Graphics
impl Spirion {
  // TBFixed
  pub fn render(&mut self, center: Vec2) {
    let mut neurons_to_remove: Vec<u32> = Vec::new();
    let mut axons_to_remove: Vec<u128> = Vec::new();

    // Step 1: build grid
    let grid = GridCell::build_spatial_grid(&self.neurons);
    // Step 2: do parallel update
    let (
      neuron_updates, 
      axon_updates
      ) = parallel_neuron_step(
        &self.neurons,
        &self.axons,
        &grid,
        center,
        |id, c| self.center_force(id, c),
        |a, b| self.spring_force(a, b),
    );

    // Step 3: apply calculated changes normally for both
    for neuron_changes in neuron_updates {
        if let Some(neuron) = self.neurons.get_mut(&neuron_changes.id) {
          if neuron.check_to_kill(neuron.has_input(&self.input_ids)) {
            neurons_to_remove.push(neuron_changes.id);
            continue;
          }
          neuron.pos = neuron_changes.new_position;
          neuron.update(self.clock);
        }
    }

    for axon_changes in axon_updates {
        if let Some(axon) = self.axons.get_mut(&axon_changes.id) {
            axon.update_happyness(axon_changes.new_happyness);
        }
    }

    // Step 4: Update Axons + Draw
    for (&id, axon) in self.axons.iter() {
      if axon.strength == 0 {
        axons_to_remove.push(id);
      }
      self.draw_axon(axon);
    }

    // Step 5: Draw neurons
    for neuron in self.neurons.values() {
        neuron.draw();
    }
  }
  
  fn draw_axon(&self, axon:&Axon) {
    let (source_id, sink_id, color) = axon.get_to_draw();
    let (source, sink) = (
      self.neurons.get(&source_id),
      self.neurons.get(&sink_id),
    );
    let mut x = 0.0;
    let mut y = 0.0;
    let mut sinkx = 0.0;
    let mut sinky = 0.0;

    if let Some(source_pos) = source {
      x = source_pos.pos.x;
      y = source_pos.pos.y;
    }

    if source_id == 0 {
     x = screen_width()/2.0; y = screen_height()
    }

    if let Some(sink_pos) = sink {
      sinkx = sink_pos.pos.x;
      sinky = sink_pos.pos.y;
    }

    draw_line (
      x,
      y,
      sinkx,
      sinky,
      2.0,
      color,
    );
  }
}






impl Spirion {
  fn no_more_outputs(&mut self, neuron_id: u32) {
    if let Some(neuron) = self.neurons.get(&neuron_id) {
      if neuron.is_output {panic!("outputs never have output axions")}
      if let Some(roll) = neuron.roll_save_check(false) {
          // Create new connections
          for _ in 0..roll {
            let sink_id = *self.neurons.keys().nth(rand::gen_range(0,self.neurons.len())).unwrap();
            if sink_id != neuron_id {
              self.add_axon(neuron_id, sink_id);
            }
          }
        } else {
          // Commit suicide
          self.remove_neuron(neuron_id);
      }
    }
  }
  
  fn add_neuron(&mut self) -> u32 {
    self.num_of_neurons +=1;
    let id = self.neurons.keys().max().unwrap_or(&0) + 1; // Generate a unique ID
    self.neurons.insert(id, Neuron::new(false));
    id
  }
  fn spawn_new_neuron(&mut self, inp_num:usize, out_num:usize) -> u32 {
    let neuron = Neuron::new(false);
    let id = self.neurons.keys().max().unwrap_or(&0) + 1; // Generate a unique ID
    self.neurons.insert(id, Neuron::new(false));
    self.num_of_neurons += 1;

    while inp_num > neuron.input_axons.len() {
      let source_id = gen_range(1, self.neurons.len() as u32);
      if let Some(source) = self.neurons.get_mut(&source_id) {
        self.add_axon(source_id, id);
      }
    }
    while out_num > neuron.output_axons.len() {
      let sink_id = gen_range(1, self.neurons.len() as u32);
      if let Some(source) = self.neurons.get_mut(&sink_id) {
        self.add_axon(sink_id, id);
      }
    }
    id
  }
  fn add_output(&mut self) -> u32 {
    self.num_of_neurons +=1;
    let id = self.neurons.keys().max().unwrap_or(&0) + 1; // Generate a unique ID
    self.neurons.insert(id, Neuron::new(true));
    self.output_ids.insert(id);
    id
  }
  
  fn add_axon(&mut self, source_id: u32, sink_id: u32) -> u128 {
    self.num_of_axons +=1;
    let id = self.axons.keys().max().unwrap_or(&0) + 1; // Generate a unique ID
    let axon = Axon::new(source_id, sink_id, id, false);
    self.axons.insert(id, axon);

    // Update neuron connections
    if let Some(source_neuron) = self.neurons.get_mut(&source_id) {
      source_neuron.output_axons.push(id);
    }
    if let Some(sink_neuron) = self.neurons.get_mut(&sink_id) {
      sink_neuron.input_axons.push(id);
    }

    id
  }
  fn add_input(&mut self, sink_id:u32) -> u128 {
    self.num_of_axons +=1;
    let id = self.axons.keys().max().unwrap_or(&0) + 1; // Generate a unique ID
    let input = Axon::new(0,sink_id, id, true);
    self.axons.insert(id, input);

    // Update neuron connections
    if let Some(sink_neuron) = self.neurons.get_mut(&sink_id) {
      sink_neuron.input_axons.push(id);
    } else {dbg!(sink_id);}
    // Put it in the brain
    self.input_ids.insert(id);
    id
  }

  fn remove_neuron(&mut self, neuron_id: u32) {
      if let Some(neuron) = self.neurons.remove(&neuron_id) {
          // Remove all input axons
          self.num_of_neurons = self.num_of_neurons.saturating_sub(1);
          for axon_id in neuron.input_axons {
              self.remove_axon(axon_id);
          }
          // Remove all output axons
          for axon_id in neuron.output_axons {
              self.remove_axon(axon_id);
          }
      } else {panic!("trying to remove a non-existent neuron???")}
  }
  fn remove_axon(&mut self, axon_id: u128) {
    if let Some(axon) = self.axons.remove(&axon_id) {
      // Remove axon from source neuron's output list
      self.num_of_axons = self.num_of_axons.saturating_sub(1);
      if let Some(source_neuron) = self.neurons.get_mut(&axon.id_source) {
        source_neuron.output_axons.retain(|&id| id != axon_id);
      }
      // Remove axon from sink neuron's input list
      if let Some(sink_neuron) = self.neurons.get_mut(&axon.id_sink) {
        sink_neuron.input_axons.retain(|&id| id != axon_id);
      }
    }
  }

}


/*
Current Plan and work:
- set up a system to input a vec every tick tied to the individual outputs
- thinking of setting up and connecting a game of pong for test-casing inputs + outputs
- 5x5 grid, one movable 2x1 paddle, a ball that just bounces back and forth
- chaos and reset any time the ball hits the wall, order any time the ball hits the paddle


current issues:
- For some reason, inputs are directly connecting to outputs
- The frezzing issue only occurs to non-outputs not connected to inputs
*/