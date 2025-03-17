// need to finish setting up the input
use macroquad::{math::Vec2,rand,prelude::{screen_width,screen_height}};
pub struct Input {
  pub id:u32, // name, basiclly
  pub position: Vec2, // Position on the screen
  pub tick: u32, // how long since firing, max at 5
  
  // This space will hold the 'plug in' for where the input will be

  pub neurons: Vec<u32>, // the neurons it connects to, no axion needed, kind is an axion tbh
}

impl Input {
  pub fn new(id:u32) -> Self {
    Self {
      id,
      position:Vec2::new(rand::gen_range(0.0+20.0,screen_width()-20.0), rand::gen_range(0.0+10.0,screen_height()-10.0)),
      tick:0,
      neurons:Vec::new(),
    }
  }
  pub fn reset(&mut self) {
    self.tick = 0;
  }
  pub fn fire(&self) -> Vec<u32> {
    self.neurons.clone()
  }
  pub fn connect(&mut self, id:u32) {
    self.neurons.push(id);
  }


  pub fn tick(&mut self) {
    if self.tick >= 5 {self.tick=5;return}
    self.tick += 1;
  }
}