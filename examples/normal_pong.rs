




/*

README: Literally normal Pong. Game size can be changed to whichever size of game you
want to play, default is a 30 x 30 grid, where None = Some(30). Pick whatever level you
want. Higher level will be harder. Game Automaticlly increases in level at score 100.


TO RUN - Paste into terminal the following line:
cargo run --example normal_pong

*/


const GAME_SIZE:Option<usize> = None;
const GAME_LEVEL:f32 = 1.0;








use macroquad::{prelude::*};

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
  let mut game = PongGame::new(GAME_SIZE, GAME_LEVEL);

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

    game.progress_frame();
    // Draw Game
    game.draw();
    if game.score >= 10 {game.level_up();}
    // Draw FPS and other info
    draw_text(
      &format!("Score: {}", game.score),
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
    if self.rows <= row || self.cols <= col {
      return None;
    }
    Some(&self.data[row * self.cols + col])
  }
  pub fn set(&mut self, row:usize, col:usize, input:T) -> Result<(),String> {
    if self.rows <= row || self.cols <= col {
      return Err("Dumbass, how tf did you break the set fn for the matrix?".to_string())
    }
    self.data[row * self.cols + col] = input;
    Ok(())
  }

}

struct PongGame {
  current_frame:Matrix<bool>,
  ball:Ball,
  paddle_col:usize,
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


impl Ball {
  fn new(center:Vec2, level:f32) -> Self {
    let x = rand::gen_range(-2.0, 2.0);
    let y = rand::gen_range(-2.0, 2.0);
    let vel = Vec2::new(x, y) * level;    
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
  fn new(game_size:Option<usize>, level:f32) -> Self {
    let mut new = PongGame { 
      current_frame: Matrix::new(game_size.unwrap_or(30), false), 
      ball: Ball::new(Vec2 { x: screen_width()/2.0, y: screen_height()/2.0 }, level), 
      paddle_col:0,
      score: 0, 
      pixle_size: pixle_size_calculator(game_size.unwrap_or(30)),
    };
    let (row, col) = new.get_ball_pos();
    new.current_frame.set(row, col, true).unwrap_or_default();
    new.current_frame.set(0,0, true).unwrap_or_default();
    new.current_frame.set(0,1, true).unwrap_or_default();
    new
  }
  fn progress_frame(&mut self)  {
    self.move_paddle(get_move());
    let (row,col) = self.get_ball_pos();
    if let Some(_) = self.current_frame.get(
      row,
      col,
    ) {
    self.current_frame.set(
      row,
      col,
      false,
    ).unwrap_or_default();
  }


    if self.ball_hit_paddle(row, col) {
      self.ball.bounce_left_right(); 
      self.score +=1;
    }
    if (col + 1) == self.current_frame.cols || col == 0 {
      self.ball.bounce_top_bottom();
    }
    if (row + 1) == self.current_frame.rows || row == 0 {
      self.ball.bounce_left_right();
    }
    
    self.ball.forward();
    let (row,col) = self.get_ball_pos();
    if let Some(_) = self.current_frame.get(
      row,
      col,
    ) {
    self.current_frame.set(
      row,
      col,
      true,
    ).unwrap_or_default();
  }
  }
  fn move_paddle(&mut self, direction:Move) {
    if let Some(_) = self.current_frame.get(
      0,
      self.paddle_col + 1, 
    ) {
    self.current_frame.set(
      0,
      self.paddle_col,
      false
    ).unwrap_or_default();
    self.current_frame.set(
      0,
      self.paddle_col + 1,
      false
    ).unwrap_or_default();
  }
    match direction {
      Move::Down => {
        if let Some(_) = self.current_frame.get(
          0,
          self.paddle_col + 2, 
        ) {
          // If this is a valid place on the map, then:
          self.current_frame.set(
            0, 
            self.paddle_col + 1,
            true
          ).unwrap_or_default();
          self.current_frame.set(
            0, 
            self.paddle_col + 2,
            true
          ).unwrap_or_default();
          self.paddle_col += 1;

        } else { 
          self.current_frame.set(
            0,
            self.current_frame.cols - 1, 
            true
          ).unwrap_or_default(); 
          self.current_frame.set(
            0, 
            self.current_frame.cols - 2, 
            true
          ).unwrap_or_default();
          self.paddle_col = self.current_frame.cols - 2;                                   
        }
      },
      Move::Up => {
        if let Some(_) = self.current_frame.get(
          0,
          self.paddle_col.saturating_sub(1), 
        ) {
          // If this is a valid place on the map, then:
          self.current_frame.set(
            0, 
            self.paddle_col.saturating_sub(1), 
            true
          ).unwrap_or_default();
          self.current_frame.set(
            0, 
            self.paddle_col.saturating_sub(1) + 1,
            true
          ).unwrap_or_default();
          self.paddle_col = self.paddle_col.saturating_sub(1);

        } else { 
          self.current_frame.set(
            0,
            0, 
            true
          ).unwrap_or_default(); 
          self.current_frame.set(
            0, 
            1, 
            true
          ).unwrap_or_default();
          self.paddle_col = 0;                                   
        }
      },
      Move::None => {}
    }
    if let Some(_) = self.current_frame.get(
    0,
    self.paddle_col + 1, 
    ) {
      self.current_frame.set(
        0,
        self.paddle_col,
        true
      ).unwrap_or_default();
      self.current_frame.set(
        0,
        self.paddle_col + 1,
        true
      ).unwrap_or_default();
    }
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
 
  fn get_ball_pos(&self) -> (usize,usize) {
    let ballx = self.ball.pos.x;
    let bally = self.ball.pos.y;
    (self.pixle_to_grid(ballx), self.pixle_to_grid(bally))
    // x->col, y->row
  }
  fn ball_hit_paddle(&self, row:usize, col:usize) -> bool {
    if row != 1 {return false;}
    if let Some(&check) = self.current_frame.get(0, col) {
      return check;
    } 
    panic!("Ball left grid");
  }
  fn pixle_to_grid(&self, pixle:f32) -> usize {
    (pixle/self.pixle_size).floor() as usize
  }
  fn level_up(&mut self) {
    self.score = 0;
    self.ball.vel += 1.0;
  }
}
fn pixle_size_calculator(game_size:usize) -> f32 {
  if game_size == 0 {panic!("Chosen game size is too large");}
  let smallest = std::cmp::min(screen_height().round() as i32, screen_width().round() as i32);
  if smallest > std::cmp::min(smallest, game_size as i32) {
    return (smallest as f32/game_size as f32) as f32
  } else { panic!("Chosen game size is too large") }
}
fn get_move() -> Move {
  if is_key_released(KeyCode::Down) {
    return Move::Down;
  }
  if is_key_released(KeyCode::Up) {
    return Move::Up;
  }
  
  
  Move::None
}


