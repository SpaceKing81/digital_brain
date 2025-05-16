use macroquad::{prelude::*};
use digital_brain::Brain;
use digital_brain::MAX_THRESHOLD;

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
  let mut game = PongGame::new(5, inputs, outputs);
  
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
    let direction = game.output_to_moves(outputs);
    // Drawing a frame
    { 
    
    // Clear the screen
    clear_background(BLACK);


    match game.progress_frame(direction) {
      Reward::Pain => {brain.pain(None);},
      Reward::Plesure => {brain.reward(None);},
      Reward::Null => {continue;},
    }
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
  output_list:Vec<u32>,
  ball:Ball,
  paddle_row:usize,
  score:usize,
  pixle_size:f32,
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
enum Reward {
  Null,
  Pain,
  Plesure,
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
  fn new(game_size:usize, input_list: Vec<u128>, output_list:Vec<u32>) -> Self {
    PongGame { 
      current_frame: Matrix::new(game_size, false), 
      input_list, 
      output_list,
      ball: Ball::new(Vec2 { x: screen_width()/2.0, y: screen_height()/2.0 }), 
      paddle_row:0,
      score: 0, 
      pixle_size: pixle_size_calculator(game_size),
    }
  }
  fn progress_frame(&mut self, direction:(Move,usize)) -> Reward {
    let mut score = Reward::Null;
    self.move_paddle(direction);
    let (row,col) = self.get_ball_pos();
    if self.ball_hit_paddle(row, col) {self.ball.bounce_left_right(); score = Reward::Plesure}
    if (row + 1) == self.current_frame.rows || row == 0 {
      self.ball.bounce_top_bottom();
    }
    if (col + 1) == self.current_frame.cols || row == 0 {
      self.ball.bounce_left_right();
    }
    if col == 0 { self.score +=1; score = Reward::Pain; }
    self.ball.forward();
    
    score
  }
  fn draw(&self) {
    let length = self.pixle_size;
    for xcell in 0..self.current_frame.rows {
      for ycell in 0..self.current_frame.cols {
        if let Some(state) = self.current_frame.get(xcell, ycell) {
          let color = if *state {WHITE} else {BLACK};
          draw_rectangle((xcell as f32) * length, (ycell as f32) * length, length, length, color);
        }}}
  }
  fn frame_to_inputs(&self) -> Option<Vec<(u128,i32)>> {
    let current_data: &Vec<bool> = &self.current_frame.data;
    let inputs: &Vec<u128> = &self.input_list;
    if current_data.len() != inputs.len() { panic!("The input-length and data length are different sizes") }
    let mut outputs:Vec<(u128,i32)> = Vec::new();
    for idx in 0..inputs.len() {
      if current_data[idx] { 
        outputs.push((
          inputs[idx],
          MAX_THRESHOLD
        ));
      }
    }
    if outputs.is_empty() {return None;}
    Some(outputs)
  }
  fn move_paddle(&mut self, direction:(Move,usize)) {
    match direction.0 {
      Move::Down => {
        if let Ok(_) = self.current_frame.set(
          self.paddle_row + direction.1 + 1, 
          0, 
          true
        ) {
          // If this is a valid place on the map, then:
          self.current_frame.set(
            self.paddle_row + direction.1, 
            0, 
            true
          ).unwrap_or_default();
          self.paddle_row = self.paddle_row + direction.1

        } else { 
          self.current_frame.set(
            self.current_frame.rows - 1, 
            0, 
            true).unwrap_or_default(); 
          self.current_frame.set(
            self.paddle_row + direction.1 - 2, 
            0, 
            true
          ).unwrap_or_default();
          self.paddle_row = self.current_frame.rows - 1;                                   
        }
      },
      Move::Up => {
        if let Ok(_) = self.current_frame.set(
          self.paddle_row.saturating_sub(direction.1),
          0, 
          true
        ) {
          // If this is a valid place on the map (and it should be always)
          self.current_frame.set(
            self.paddle_row.saturating_sub(direction.1),
            0, 
            true
          ).unwrap_or_default();

        } else {unreachable!("Somehow, the matrix doesn't have a (0,0) cord?")}
      },
      Move::None => return,
    }
  }
  fn get_ball_pos(&self) -> (usize,usize) {
    let ballx = self.ball.pos.x;
    let bally = self.ball.pos.y;
    (self.pixle_to_grid(bally), self.pixle_to_grid(ballx))
    // x->col, y->row

  }
  fn ball_hit_paddle(&self, row:usize, col:usize) -> bool {
    if col != 1 {return false;}
    if let Some(&check) = self.current_frame.get(row, 0) {
      return check;
    } 
    panic!("Ball left grid");
  }
  fn output_to_moves(&self, output:Vec<u32>) -> (Move,usize) {
    let mut up: i32 = 0;
    for i in output {
      // Move Up
      if i == self.output_list[0] {up += 1;}

      // Move Down
      if i == self.output_list[1] {up -= 1;}
    }

    if up == 0 {return (Move::None, 0);}
    if up.is_positive() {return (Move::Up, up as usize);}
    if up.is_negative() {return (Move::Down, up.abs() as usize);}

    panic!("Just...how can a number not be pos, neg, or 0???")
  }
  fn pixle_to_grid(&self, pixle:f32) -> usize {
    (pixle/self.pixle_size).floor() as usize
  }
}
fn pixle_size_calculator(game_size:usize) -> f32 {
  if game_size == 0 {panic!("Chosen game size is too large");}
  let smallest = std::cmp::min(screen_height().round() as i32, screen_width().round() as i32);
  if smallest > std::cmp::min(smallest, game_size as i32) {
    return (smallest as f32/game_size as f32) as f32
  } else { panic!("Chosen game size is too large") }
}