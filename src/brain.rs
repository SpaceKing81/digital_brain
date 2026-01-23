use std::collections::{HashMap, HashSet};

// File saving
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, sink};
use serde::{Serialize, Deserialize};
use bincode::{self, deserialize};


use macroquad::{
  // color::*, 
  color::{GRAY}, rand, shapes::*, window::{screen_height, screen_width}
};

use crate::{
  //
  axon::Axon, consts::{self, *}, internal_consts::{self, *}, neuron::{self, Neuron}, pos::Pos,
  //
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Spirion {
  pub clock:u128,

  neurons: HashMap<u32, Neuron>,
  axons: HashMap<u128,Axon>,
  output_ids: HashSet<u32>,
  input_ids: HashSet<u128>,

  num_of_neurons: u32,
  num_of_axons: u128,

  active_neurons:HashSet<u32>,

  limiter: bool,
  displayed: bool,
}
// Public
impl Spirion {
  /// New brain with specifed inputs and outputs, optional number of Neurons.
  /// Default is 500 Neurons
  pub fn spin_up_new(num_neurons: Option<u32>, num_input: u128, num_output: u32, window:bool) -> (Self, Vec<u128>, Vec<u32>) {
    let num_neurons = num_neurons.unwrap_or(500);
    // Step 0: Create Brain
    let mut brain = Self::new();
    if window {brain.displayed = true;}
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
          brain.find_happier_space(*id);
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
  
  /// Allows Spirion to operate at max speeds regardless of input
  pub fn remove_limits(&mut self) {
    self.limiter = false;
  }

  /// Forces a hard-stop limit to spirion's thinking speed to better align with reality
  pub fn limit(&mut self) {self.limiter = true}

  /// Ticks over the brain simulation for however many specified ticks, with a default of 1 iteration.
  /// No input, however all the output id's are spat out as an Option<Vec>
  pub fn tick(&mut self, num_iterations:Option<u32>) -> Option<Vec<u32>> {
    let mut output = Vec::new();
    if num_iterations == Some(0) {panic!("Need a number larger then 0 iterations")}
    // Cannot render more at once with no input then a single standard_deviation without a cascading upshooting of happyness values!!???
    // I'm reading this months later and just like... wtf??
    for _ in 0..(
      std::cmp::min(
        num_iterations.unwrap_or(1),
        (ONE_STANDARD_DEV_THRESHOLD - 1 ).abs() as u32
      )
    ) {
      // one tick passes
      self.clock += 1;
      if self.limiter {
        macroquad::experimental::coroutines::wait_seconds(1.0);
      }

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

          // Find the requisate number of input-output axon additions
          // For future me, this is taking a pre-calculated number of how many connections to drop if neg,
          // or add if pos. the num_in is for inputs, num_out for outputs
          if num_in.is_negative() {
            for _ in 0..num_in.abs() {
              let index = rand::gen_range(0, neuron.input_axons.len());
              let id = neuron.input_axons[index];
              axons_to_remove.push(id);
            }
          } else {
            let mut sources = Vec::new();
            for _ in 0..num_in {
              let source = rand::gen_range(1, self.num_of_neurons);
              sources.push(source);
            }
            axons_to_add_sink.push((neuron_id,sources));
          }
          if num_out.is_negative() {
            for _ in 0..num_out.abs() {

              let index = rand::gen_range(0, neuron.output_axons.len());
              let id = neuron.output_axons[index];
              axons_to_remove.push(id);
            }
          } else {
            let mut sinks = Vec::new();
            for _ in 0..num_out {
              let sink = rand::gen_range(1, self.num_of_neurons);
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
      for neuron_id in neurons_to_remove {self.remove_neuron(neuron_id);}

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
  /// Makes it feel good, hopefully emulating pleasure
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

  /// Goes through the whole brain and cleans up neurons.
  /// Updates every neuron, culls any dead ones, and resets the brain counter
  /// Does not touch the active_neuron list, and does not run a tick cycle
  pub fn cleanse(&mut self) {
    todo!();
  }


  /// Allows for saving the current values incoded in Spirion as a .bin file for future running
  /// (in....complete?)
  pub fn save_as_bin(&self, pathname:&str) -> std::io::Result<()> {
    let file = File::create(pathname)?;
    let writer = BufWriter::new(file);
    bincode::serialize_into(writer, &self).unwrap();
    println!("Succsful file save at: {}", pathname);
    Ok(())
  }

  /// Allows for building a Spirion using specified values stored in a .bin file from a 
  /// previous running. Returns Spirion, inputs and outputs (in...complete?)
  pub fn build_from_bin(full_pathname:&str) -> (Self, Vec<u128>, Vec<u32>) {
    let file = File::open(full_pathname).unwrap();
    let reader = BufReader::new(file);
    let brain: Self = bincode::deserialize_from(reader).unwrap();

    let input:Vec<u128> = (&brain).input_ids.iter().copied().collect();
    let output:Vec<u32> = (&brain).output_ids.iter().copied().collect();
    
    (brain, input, output)
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
      limiter:true,
      displayed:false,
    }
  }
}

/// Graphics
impl Spirion {
  /*
    Thoughts:
    - Only the active stuff is gonna show up
    - I want a sort of fade thing, so that it remembers which ones were fired last
    - A sort of effect where the neurons fade out of existence as they dont get fired
    - thinking 10 hashsets, each has the previous lineups up to 10 ticks, then replac
    */
  
  /// When called, will draw all the neurons that are in the tbfired list, 
  /// along with their connected axon outputs
  pub fn render(&mut self) {
    if !self.displayed {return}
    for &neuron_id in &self.active_neurons {
        if let Some(neuron) = self.neurons.get(&neuron_id) {
          // Draw Neuron
          neuron.draw();
          
          // Draw the output axons
          let neuron_pos = neuron.get_pos();        
          for &out_axon in &neuron.output_axons {
            self.draw_output_axon(&neuron_pos, out_axon);
          }
        } else {dbg!(neuron_id);}
    }
  }
  
  fn draw_output_axon(&self, neuron_pos:&Pos, axon_id:u128) {
    let (x1,y1) = (neuron_pos.x,neuron_pos.y);
    if let Some(axon) = self.axons.get(&axon_id) {
      // Pick Color
      let mut color = GRAY;
      if axon.strength.is_negative() {color = internal_consts::AXON_NEG_COLOR;}
      if axon.strength.is_positive() {color = internal_consts::AXON_POS_COLOR;}
      if axon.is_input() {color = internal_consts::AXON_INPUT_COLOR;}
      // Second Position Value
      let mut pos2:Pos = Pos::zero();
      // This is the line that need to change between the two functions
      if let Some(neuron2) = self.neurons.get(&axon.id_sink) {
        pos2 = neuron2.get_pos();
      }
      let (x2,y2) = (pos2.x, pos2.y);
      // Draw it
      draw_line (
        x1,
        y1,
        x2,
        y2,
        2.0,
        color,
      ); 
    } else {dbg!("Output Axon in neuron does not exist. Drawing Output axon");}
  }
}




/// Dynamic Brain changing
impl Spirion {
  fn find_happier_space(&mut self, neuron_id: u32) {
    /*
    Use to be the "no more outputs" fn
    The idea of this is that it will seperate the death and the search
    If a neuron is too unhappy, it'll get picked up by the garbage collector
    And fully get killed. This is for the ones that get through but still lonely
     */
    todo!();
  }
  
  /*
  TODO: Optimize current setup for adding-removing neurons + connections
  Logic: 
  - Unhappy neurons (need to massivly fix that logic too to allow for much broader resilliences. Can't have neurons
  dying due to little suprises) will drop connnections. Neurons with no connections will be removed from the brain.
  No redo, or capacity to stay alive if its happy, just straight in the trash.
  - Need to periodicly run full brain cleanses as well, or allow the capability for people to do that, like brain.cleanse(). 
  That will allow the entire thing to update fully, and reset the internal clock.
  Happy neurons will reproduce. They will spawn copies of themselves (somehow) which will have identical connections to
  all the neurons. Which means adding the new neuron to all the inputs and all the output neurons. Obvi special cases (IO stream)
  are exempt. Additionally, not-unhappy neurons (wording intentional) can induce axons to replicate, causing the new 
  axons to spontaniously form between them and random neurons.
  Summery: 
  Unhappy cull worst axons until no longer unhappy (except IO stream), not-unhappy add output axons to random neurons, 
  Happy neurons induce replication of that neuron and all its connections (mutations??), connectionless neurons (in
  input) are killed.

   */
  fn add_neuron(&mut self) -> u32 {
    self.num_of_neurons +=1;
    let id = self.neurons.keys().max().unwrap_or(&0) + 1; // Generate a unique ID
    self.neurons.insert(id, Neuron::new(false, self.displayed));
    id
  }
  fn spawn_new_neuron(&mut self, inp_num:usize, out_num:usize) -> u32 {
    let neuron_new = Neuron::new_with_time(self.displayed, self.clock);
    let id_new = self.neurons.keys().max().unwrap_or(&0) + 1; // Generate a unique ID
    self.neurons.insert(id_new, Neuron::new(false, self.displayed));
    self.num_of_neurons += 1;

    /*
    bug notes. There is a strange bug happening here. the logic should be increasing the number of
    connections until it surpasses a threshold, and then it continues. Gets stuck in an infinite loop
    because its calling spawn_new(its a new neuron that needs friends) and its just not getting any
    more connections. I need to still check the axon list and how thats changing, but the weirdest part
    is that its running fine through all the checks but seems to not be affecting anything at all...

    idk, going to bed. The trail of dbg can be cleaned up later. night
    
    Fuck you past me. Fuck you.

    Found the bug. The length is just perpetually returning 0 despite having values in it?
    I don't know why this is even a problem in the first place, the .len() is broken ig?
    
    Bug no longer there as long as I don't check the length...? as of 1/22/26
     */
    for _ in 0..inp_num
    // while inp_num > neuron_new.input_axons.len() 
     {
      // Adding to the inputs, so the new is the sink
      let source_id = rand::gen_range(1, self.neurons.len() as u32);
      if let Some(_) = self.neurons.get_mut(&source_id) {
        self.add_axon(source_id, id_new);
      }
    }
    for _ in 0..out_num
    // while out_num > neuron_new.output_axons.len()
    {
      // Adding to the outputs, so the new is the source
      let sink_id = rand::gen_range(1, self.neurons.len() as u32);
      if let Some(_) = self.neurons.get_mut(&sink_id) {
        self.add_axon(id_new, sink_id);
      }
    }
    id_new
  }
  fn add_output(&mut self) -> u32 {
    self.num_of_neurons +=1;
    let id = self.neurons.keys().max().unwrap_or(&0) + 1; // Generate a unique ID
    self.neurons.insert(id, Neuron::new(true, self.displayed));
    self.output_ids.insert(id);
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
  fn add_axon(
    &mut self, 
    source_id: u32, 
    sink_id: u32,
  ) -> u128 {
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

*/