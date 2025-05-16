use std::sync::RwLockWriteGuard;

use macroquad::{input, prelude::*, rand::rand};
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
  let (mut brain,inputs, outputs) = Brain::spin_up_new(1500, 25, 2);
  let mut game = PongGame::new(5, inputs);
  
  let initial_pos: Option<Vec<(u128,i32)>> = game.frame_to_inputs();
  brain.brain_input(initial_pos);

  // Main loop
  loop {
    // Handle Ending
    if is_key_down(KeyCode::Escape) {
      println!("Terminating");
      break;
    }
    // Brain thinking
    let outputs = brain.tick(Some(29));
    let direction = Move::output_to_moves(outputs);
    // Drawing a frame
    { 
    
    // Clear the screen
    clear_background(BLACK);


    game.progress_frame(direction);
    // Draw Game
    game.draw();
    brain.brain_input(game.frame_to_inputs());

    // Draw FPS and other info
    // draw_text(
    //   &format!("Hello world"),
    //   20.,
    //   20.,
    //   20.,
    //   WHITE,
    // );
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

}

struct PongGame {
  current_frame:Matrix<bool>,
  input_list:Vec<u128>,
  ball:Ball,
  score:usize,
  pixle_size:f32,
  bottom_right:Vec2,
}
struct Ball {
  pos:Vec2,
  vel:Vec2,
}
#[derive(Clone, Copy)]
enum Move {
  Up,
  Down,
  None,
}

impl Move {
  fn output_to_moves(outputs:Vec<u32>) -> (Self, usize) {todo!();}
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
  fn new(game_size:usize, input_list: Vec<u128>) -> Self {
    let edge = ((game_size as f32)-1.0) * pixle_size_calculator(game_size);
    PongGame { 
      current_frame: Matrix::new(game_size, false), 
      input_list, 
      ball: Ball::new(Vec2 { x: screen_width()/2.0, y: screen_height()/2.0 }), 
      score: 0, 
      pixle_size: pixle_size_calculator(game_size),
      bottom_right: Vec2::new(edge, edge),
    }
  }
  fn progress_frame(&mut self, direction:(Move,usize)) {
    self.move_paddle(direction);
    let (row,col) = self.check_ball_pos();
    if (row + 1) == self.current_frame.rows || row == 0 {
      self.ball.bounce_top_bottom();
    }
    if (col + 1) == self.current_frame.cols || row == 0 {
      self.ball.bounce_left_right();
    }
    self.ball.forward();
  }
  fn shift_paddle(&mut self, direction:(Move,usize)) {todo!();}
  fn move_ball(&mut self) {todo!();}
  fn draw(&self) {
    let length = self.pixle_size;
    for xcell in 0..self.current_frame.rows {
      for ycell in 0..self.current_frame.cols {
        if let Some(state) = self.current_frame.get(xcell, ycell) {
          let color = if *state {WHITE} else {BLACK};
          draw_rectangle((xcell as f32) * length, (ycell as f32) * length, length, length, color);
        }}}
  }
  fn frame_to_inputs(&self) -> Option<Vec<(u128,i32)>> {todo!();}
  fn move_paddle(&mut self, direction:(Move,usize)) {}
  fn check_ball_pos(&self) -> (usize,usize) {todo!()}
}

fn pixle_size_calculator(game_size:usize) -> f32 {
  if game_size == 0 {panic!("Chosen game size is too large");}
  let smallest = std::cmp::min(screen_height().round() as i32, screen_width().round() as i32);
  if smallest > std::cmp::min(smallest, game_size as i32) {
    return (smallest as f32/game_size as f32) as f32
  } else { panic!("Chosen game size is too large") }
}