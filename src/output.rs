use macroquad::{
  color::*, math::Vec2, rand, shapes::*,
  window::{screen_width,screen_height},
};

const FIRED:Color = Color::new(0.0, 0.25, 0.5, 1.0);
const WAITING:Color = Color::new(0.0, 0.5, 1.0, 1.0);

pub struct Output {

  pub id:u32, // name, basiclly
  pub position: Vec2, // Position on the screen
  pub tick: u32, // how long since firing, max at 5
  
  // This space will hold the 'plug in' for where the output will be

  pub input_axions: Vec<u128>, // the neurons that connect to it, no axion needed, basiclly an axion
}

impl Output {
  fn new(id:u32) -> Self {
    Self {
      id,
      position:Vec2::new(rand::gen_range(0.0+20.0,screen_width()-20.0), rand::gen_range(0.0+10.0,screen_height()-10.0)),
      tick:0,
      input_axions:Vec::new(),
    }
  }


  pub fn update(&mut self) {}
  pub fn draw(&self) {
    let color = if self.tick < 5 {FIRED} else {WAITING};
    draw_circle(self.position.x, self.position.y, 10.0, color); // Wintery blue
  }
}