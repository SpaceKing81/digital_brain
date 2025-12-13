use std::collections::HashSet;
use serde::{Serialize, Deserialize};
// use std::collections::HashMap;
use macroquad::{
  color::*, rand, shapes::*,
  window::{screen_width,screen_height},
};

use crate::internal_consts::*;
use crate::consts::*;
use crate::pos::*;




/// Single neuron
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Neuron {
  // id:u32, // name, basiclly
  position: Pos, // Position on the screen
  base_threshold:i32,
  threshold:i32, // threshold to fire
  pub happyness:u32, // how happy it is with the firing frequency, 0 is happiest
  
  pub is_output: bool,

  pub inputs:Vec<i32>, // total inputs for this tick, post weight
  input_memory:Vec<i32>, // Memory of previous values, currently 5

  pub output_axons: Vec<u128>, 
  pub input_axons: Vec<u128>,

  tick_last_fired:u128, // the time it fired last
  pub delta_t:u32, // how long since last fire
  avg_t:u32, // average time since last firing
  
} 

// General
impl Neuron {
  /// Makes a new neuron
  pub fn new(is_output:bool, visualizable:bool) -> Self {
    // If the brain is made to be displayed, then true, otherwise its false
    Neuron {
        // id,
        position: if visualizable {
          Pos::new(
            rand::gen_range(0.0+20.0,screen_width()-20.0), 
            rand::gen_range(0.0+10.0,screen_height()-10.0)
          )
        } else {
          Pos::zero()
        },
        happyness:25,
        base_threshold:50,
        threshold:50,
        is_output,
        
        input_memory:vec![0,0,0,0,0],
        inputs:Vec::new(),

        output_axons: Vec::new(),
        input_axons: Vec::new(),
        
        tick_last_fired:0,
        delta_t:0,
        avg_t:0,
    }
  }
  pub fn new_with_data(
    pos:Option<Pos>, 
    happyness:Option<u32>,
    base_threshold:Option<i32>,
    threshold:Option<i32>,
    is_output:Option<bool>,
    
    input_memory:Option<Vec<i32>>,
    inputs:Option<Vec<i32>>,

    input_axons:Option<Vec<u128>>,
    output_axons:Option<Vec<u128>>,
    
    tick_last_fired:Option<u128>,
    delta_t:Option<u32>,
    avg_t:Option<u32>,
  ) -> Self {
    let position = pos.unwrap_or(
      Pos::new(rand::gen_range(20.0,screen_width()-20.0), 
        rand::gen_range(10.0,screen_height()-10.0)
      ));

    Neuron {
        // id,
        position,
        happyness:happyness.unwrap_or(25),
        base_threshold:base_threshold.unwrap_or(50),
        threshold:threshold.unwrap_or(50),
        is_output: is_output.unwrap_or(false),
        
        input_memory:input_memory.unwrap_or(vec![0,0,0,0,0]),
        inputs:inputs.unwrap_or(Vec::new()),

        input_axons:input_axons.unwrap_or(Vec::new()),
        output_axons:output_axons.unwrap_or(Vec::new()),
        
        tick_last_fired:tick_last_fired.unwrap_or_default(),
        delta_t:delta_t.unwrap_or_default(),
        avg_t:avg_t.unwrap_or_default(),
    }
  }
  /// Rolls a save check to see if it should die or gets another chance at life, and if so how many.
  /// Only relies on happyness value, but only really used if it doesnt have any outputs or inputs left.
  pub fn roll_save_check(&self, output:bool) -> Option<i32> {
    if output {return Some(rand::gen_range(10,20))}
    let roll = rand::gen_range(0,MAX_HAPPY_VALUE/2);
    if roll + self.happyness < MAX_HAPPY_VALUE/5 {
      return Some(rand::gen_range(10,20))
    }
    None
  }
}
// Update
impl Neuron {
  /// Honestly useless, just sets the tick to 0, returns time since last fired
  pub fn fired(&mut self) -> u32 {
    let delta_t = self.delta_t;
    self.delta_t = 0;
    delta_t
  }
  
  /// Housekeeping stuff, memory management, time updating, basic universal update.
  /// Updates everything that needs to be refreshed whenever it becomes an active neuron.
  /// Returns how may (inputs to add, outputs to add)
  pub fn update(&mut self, time:u128) -> (i16,i16) {
    // Clock update, updates action as needed
    let old_time = self.delta_t;
    self.tick(time);

    // Update threshold values
    self.update_threshold();
    self.math();
    // Update the memory as fit
    if old_time + 5 <= self.delta_t {
      // Voids memory and replaces it with the current update scheme
      self.forget(None);
    } else {
      // Updates memory as accurate to the times
      self.forget(Some((self.delta_t-old_time)as usize));
    }



    todo!();
  }
  pub fn want_to_reproduce(&self) -> bool {
    // needs to be happy enough to want to reproduce
    self.happyness < (MAX_HAPPY_VALUE/4)
  }

  fn tick(&mut self, time:u128) {
    self.delta_t = (time - self.tick_last_fired) as u32;
  } 
  fn math(&mut self) {
    let t = self.delta_t as i32;
    let w = self.avg_t as i32;
    let delta = (w - t).abs();
    let weight = (delta/ONE_STANDARD_DEV_THRESHOLD).abs() as u32;
    
    // alters the reliablity by the distence from average
    self.happyness += weight;
    if weight == 0 {self.happyness = self.happyness.saturating_sub(5);}
    if weight <= 2 {self.happyness = self.happyness.saturating_sub(2)}
    self.avg_t = (self.avg_t + self.avg_t + self.delta_t) / 3;
  }
  
}

// Graphics
impl Neuron {
  /// Draws the Neuron where ever it is, gives it a color based on its firing status + output
  pub fn draw(&self) {
    // if self.position.is_nan() {print!(" Caught! ")}
    if self.is_output { draw_circle(self.position.x, self.position.y, 10.0, OUTPUT_COLOR); return;}
    let color = if self.delta_t <= 10 {YELLOW} else {GRAY};
    draw_circle(self.position.x, self.position.y, 10.0, color);
  }
  pub fn get_pos(&self) -> Pos {self.position}

}
// Output stuff
impl Neuron {
  /// Checks if the neuron wants to fire 
  pub fn ready_to_fire(&self) -> bool {
    if self.delta_t <= 5 {return false}
    let potential:i32 = self.input_memory.iter().sum();
    potential.abs() >= self.threshold
  } 
  /// Checks if the neuron should be killed
  pub fn check_to_kill(&self, has_input: bool) -> bool {
    if has_input {return false}
    if self.is_output {return false}
    if self.happyness >= MAX_HAPPY_VALUE {return true}
    if self.delta_t >= INACTIVITY_DEATH_TIME {return true}
    false
  }

  pub fn has_input(&self, input_ids: &HashSet<u128>) -> bool {
    for &i in &self.input_axons {
      if input_ids.contains(&i) {return true}
    }
    false
  }
}
// Input
impl Neuron {
  /// Either specify how many lost seconds, or complete replacement if none for memory
  fn forget(&mut self, ticks:Option<usize>) {
    let sum:i32 = self.inputs.iter().sum();
    if ticks == Some(0) {return;}
    match ticks {
      None => {
        // get the current sum value
        let mut input_memory = vec![sum];
        input_memory.append(&mut vec![0;MEMORY_SIZE-1]);
        self.inputs.clear();
      },
      Some(u) => {
        let u = if u<MEMORY_SIZE {u}else{MEMORY_SIZE};
        for _i in 0..u {self.input_memory.pop();}
        let sum:i32 = self.inputs.iter().sum();
        let mut new = vec![sum];
        new.append(&mut vec![0;u-1]);
        new.append(&mut self.input_memory);
        self.input_memory = new;
      },
    }
  }

}

// Mutate Thresholds
impl Neuron {
  /// Updates the threshold based on frequency and happyness, along with factoring in a cooldown period
  pub fn update_threshold(&mut self) {
    if self.ready_to_fire() {
      let potential:i32 = self.input_memory.iter().sum();
      let extreme_fire:bool = potential.abs() > 100;
      let u = (self.delta_t as i32) - (self.avg_t as i32);
      let w = u / ONE_STANDARD_DEV_THRESHOLD;
      if w.abs() <= 1 {
        self.happyness = self.happyness.saturating_sub(4);

      } else if w.abs() > 4 {
        if (w.is_negative() && self.base_threshold >= 70) || extreme_fire {
            self.happyness += w.abs() as u32;
        } else if w.is_negative() || extreme_fire {
            self.raise_base_threshold();
        }
        if w.is_positive() && self.base_threshold <= 30 {
          self.happyness += w.abs() as u32;
        } else if w.is_positive() {
            self.drop_base_threshold();
        }
      }
    } else {
      self.post_fire_threshold();
    }
  }

  // alters actual threshold when its right after firing
  fn post_fire_threshold(&mut self) {
    if self.delta_t >= 15 { self.set_threshold_to_normal(); }
    if self.delta_t >= 5 { self.set_threshold_to_increment(); }
    if self.delta_t < 5 { self.set_threshold_to_recovery(); }
  } 

  // ease of reading fns
  fn set_threshold_to_normal(&mut self) {
    self.threshold = self.base_threshold;
  }
  fn set_threshold_to_increment(&mut self) {
    self.threshold = std::cmp::max(self.threshold - (3 * self.delta_t as i32 - 15),0);
  }
  fn set_threshold_to_recovery(&mut self) {
    self.threshold = self.base_threshold + 30;
  }
  
  // Base Threshold shifts
  fn drop_base_threshold(&mut self) {
    let u = (self.delta_t as i32) - (self.avg_t as i32);
    let w = u / ONE_STANDARD_DEV_THRESHOLD;
    let w = w.abs();
    self.base_threshold = self.base_threshold.saturating_sub(w);
    if self.base_threshold < MIN_THRESHOLD { self.base_threshold = MIN_THRESHOLD; }
    self.post_fire_threshold();
  }
  fn raise_base_threshold(&mut self) {
    let u = (self.delta_t as i32) - (self.avg_t as i32);
    let w = u / ONE_STANDARD_DEV_THRESHOLD;
    let w = w.abs();
    self.base_threshold += w;
    if self.base_threshold > MAX_THRESHOLD { self.base_threshold = MAX_THRESHOLD; }
    self.post_fire_threshold();
  }
  
}

