use macroquad::{
  color::*, math::Vec2, rand, shapes::*,
  window::{screen_width,screen_height},
};
use crate::consts::*;

const FIRED:Color = Color::new(0.0, 0.25, 0.5, 1.0);
const WAITING:Color = Color::new(0.0, 0.5, 1.0, 1.0);

pub struct Output {

  // pub id:u32, // name, basiclly
  pub position: Vec2, // Position on the screen
  pub tick: u32, // how long since firing, max at 5
  last_tick:u128, // The last called tick
  base_threshold:i32, // threshold to fire, both pos and neg values
  
  // This space will hold the 'plug in' for where the output will be
  
  pub input_axions: Vec<u128>, // the neurons that connect to it, no axion needed, basiclly an axion
  pub inputs:Vec<i32>, // total inputs for this tick, post weight
  input_memory:Vec<i32>, // Memory of previous values
}

impl Output {
  pub fn new(id:u32) -> Self {
    Self {
      // id,
      position:Vec2::new(rand::gen_range(0.0+20.0,screen_width()-20.0), rand::gen_range(0.0+10.0,screen_height()-10.0)),
      tick:0,
      last_tick:0,
      input_axions:Vec::new(),
      base_threshold:OUTPUT_THRESHOLD,
      inputs:Vec::new(),
      input_memory:Vec::new(),
    }
  }
  pub fn update(&mut self, time:u128) {
    // Update threshold values
    // self.update_threshold();
    // self.math();

    // Update the memory as fit
    if self.last_tick + 5 <= time {
      // Voids memory and replaces it with the current update scheme
      self.forget(None);
    } else {
      // Updates memory as accurate to the times
      self.forget(Some((time-self.last_tick) as usize));
      // NOTE: DIFFERENT THEN NEURON BECAUSE NEURON UPDATES TIME FIRST, NOT LAST. DO NOT
      // LOOK AT THIS AND FIX IT BECAUSE THEY ARE SUPOSE TO BE LIKE THIS, THOUGHT WAS PUT IN
      // ALREADY. STUPID.
    }
    // Clock update, updates action as needed
    self.last_tick = time;
  }
  pub fn fired(&mut self) {
    self.tick == 0;
    self.forget(None);
  }

  pub fn draw(&self) {
    let color = if self.tick < MEMORY_SIZE as u32 {FIRED} else {WAITING};
    draw_circle(self.position.x, self.position.y, 10.0, color); // Wintery blue
  }
}

impl Output {
  /// Checks if the output should fire 
  pub fn ready_to_fire(&self) -> bool {
    if self.tick < MEMORY_SIZE as u32 {return false}
    let potential:i32 = self.input_memory.iter().sum();
    potential.abs() as i32 >= self.base_threshold
  } 

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