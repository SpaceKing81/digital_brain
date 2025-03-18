// need to finish setting up the input
use macroquad::{
  color::*, math::Vec2, rand, shapes::*,
  window::{screen_width,screen_height},
};

const FIRED:Color = Color::new(1.0, 0.5, 0.0, 1.0);
const WAITING:Color = Color::new(0.5, 0.25, 0.0, 1.0);

pub struct Input {
  pub id:u32, // name, basiclly
  pub position: Vec2, // Position on the screen
  pub tick: u32, // how long since firing, max at 5
  
  // This space will hold the 'plug in' for where the input will be

  pub output_neurons: Vec<u32>, // the neurons it connects to, no axion needed, kind is an axion tbh
}

impl Input {
  pub fn new(id:u32) -> Self {
    Self {
      id,
      position:Vec2::new(rand::gen_range(0.0+20.0,screen_width()-20.0), rand::gen_range(0.0+10.0,screen_height()-10.0)),
      tick:0,
      output_neurons:Vec::new(),
    }
  }
  pub fn reset(&mut self) {
    self.tick = 0;
  }
  pub fn fire(&self) -> Vec<u32> {
    self.output_neurons.clone()
  }
  pub fn connect(&mut self, id:u32) {
    self.output_neurons.push(id);
  }


  pub fn tick(&mut self) {
    if self.tick >= 5 {self.tick=5;return}
    self.tick += 1;
  }

  pub fn draw(&self) {
    let color = if self.tick < 5 {FIRED} else {WAITING};
    draw_circle(self.position.x, self.position.y, 10.0, color);
    // Crimson
  }
}