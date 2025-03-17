use macroquad::math::Vec2;

pub struct Output {

  pub id:u32, // name, basiclly
  pub position: Vec2, // Position on the screen
  pub tick: u32, // how long since firing, max at 5
  
  // This space will hold the 'plug in' for where the output will be

  pub input_axions: Vec<u128>, // the neurons that connect to it, no axion needed, basiclly an axion
}