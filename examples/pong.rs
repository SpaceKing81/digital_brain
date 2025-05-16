use macroquad::{prelude::*, rand::rand};
use digital_brain::Brain;

fn window_conf() -> Conf {
    Conf {
        window_title: "Pong Visual".to_owned(),
        fullscreen: false,
        window_resizable: true,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
  println!("Starting simulation...");
  
  // Main loop
  loop {
    // Handle Ending
    if is_key_down(KeyCode::Escape) {
      println!("Terminating");
      break;
    }
    // Drawing a frame
    { 

    // Clear the screen
    clear_background(BLACK);

    // Update and draw neurons and axons
    
    // Draw FPS and other info
    draw_text(
      &format!("Hello world"),
      20.,
      20.,
      20.,
      WHITE,
    );
    }
    // Render the frame
    next_frame().await;
  }
}

struct Matrix<T> {
  data:Vec<T>, // either white or black
  cols:usize,
  rows:usize,
}
impl<T> Matrix<T> {
/*
matrix
  0|1|2
  3|4|5
  6|7|8
  ->
  data
  [0,1,2,3,4,5,6,7,8]
*/
  /// Makes a new 0 index matrix. (0,0) is a11
  pub fn new(rows:usize, default: T) -> Self where T:Clone,{
    Matrix {
      data: vec![default; rows*rows],
      rows,
      cols:rows,
    }
  }
  pub fn get(&self, row:usize, col:usize) -> Option<&T> {
    if self.rows < row && self.cols < col {
      return None
    }
    Some(&self.data[row * self.cols + col])
  }
  pub fn set(&mut self, row:usize, col:usize, input:T) -> Result<(),String> {
    if self.rows < row && self.cols < col {
      return Err("Dumbass, how tf did you break the set fn for the matrix?".to_string())
    }
    self.data[row * self.cols + col] = input;
    Ok(())
  }
  pub fn shrink(&mut self) -> Result<(),String> {Ok(())}

}

struct PongGame {
  clock:usize,
  current_frame:Matrix<bool>,
  input_list:Vec<u128>,
  ball:Ball,

  score:usize,
  game_size:usize,


}
struct Ball {
  pos:Vec2,
  vel:Vec2,
}
impl Ball {
  fn new(center:Vec2) -> Self {
    let x = rand::gen_range(0.0, 2.0);
    let y = rand::gen_range(0.0, 2.0);
    let vel = Vec2::new(x, y).normalize();    
    Ball {
      vel,
      pos:center,
    }
  }
  fn forward(&mut self) {
    self.pos += self.vel;
  }
  fn bounce_top_bottom(&mut self) {
    self.vel = Vec2::new(self.vel.x, 0.0-self.vel.y);
  }
  fn bounce_left_right(&mut self) {
    self.vel = Vec2::new(0.0-self.vel.x, self.vel.y);
  }

}

impl PongGame {
  fn new() -> Self {
    todo!()
  }
}