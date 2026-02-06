




/*

README: Literally normal Snake. Game size can be changed to whichever size of game you
want to play, default is a 30 x 30 grid, where None = Some(20). Pick whatever level you
want. Higher level will be harder. Game Automaticlly increases in level you fill
the screen


TO RUN - Paste into terminal the following line:
cargo run --example normal_snake

*/


const GAME_SIZE:Option<usize> = None;
const GAME_LEVEL:f32 = 1.0;








use std::clone;

use macroquad::{prelude::*};

fn window_conf() -> Conf {
    Conf {
        window_title: "Snake Visual".to_owned(),
        fullscreen: false,
        window_resizable: true,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
  println!("Starting simulation...");
  let mut game = SnakeGame::new(GAME_SIZE, GAME_LEVEL);

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
    if game.score >= GAME_SIZE.unwrap_or(30)*GAME_SIZE.unwrap_or(30) {game.level_up();}
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
  0|1|2 row0
  3|4|5 row1
  6|7|8 row2
  ->
  data
  [0,1,2,3,4,5,6,7,8]
*/
  /// Makes a new 0 index matrix. (0,0) is a11, (1,2) is a23
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
#[derive(Clone, Debug, PartialEq, Copy)]
struct Coords {
  row:usize,
  col:usize,
}
impl Coords {
  fn add(&mut self, (row,col):(usize,usize)) {
    self.row += row;
    self.col += col;
  }
  fn sub(&mut self, (row,col):(usize,usize)) {
    self.row.saturating_sub(row);
    self.col.saturating_sub(col);
  }
  fn is_any_same(&self, row:usize, col:usize) -> bool {
    self.row == row || self.col == col
  }
  fn is_any_greater_than(&self, row:usize, col:usize) -> bool {
    self.row > row || self.col > col
  }
  
  const ZERO:Self = Self {row:0,col:0};
}


struct SnakeGame {
  current_frame:Matrix<f32>,
  apple_gradient:Matrix<f32>,
  snake:Snake,
  apple:Coords,
  
  score:usize,
  pixle_size:f32,
}
struct Snake {
  head:Coords,
  dir:Dir,
  path:Vec<Coords>,
  length:u32,
}



#[derive(Clone, Copy)]
enum Dir {
  Left,
  Right,
  Up,
  Down,
  None,
}


impl Snake {
  fn new(center:Coords, level:f32) -> Self {
    // let x = rand::gen_range(-1.0, 1.0);
    // let dir = if x.is_sign_positive() {
    //   Dir::Up
    // } else {
    //   Dir::Down
    // };
    let dir = Dir::Up;
    Self {
      dir,
      head:center,
      path:vec![center],
      length:1,
    }
  }
  fn forward(&mut self) {
    let dir = self.dir;
    self.path.remove(0);
    self.path.push(self.head);
    match dir {
      Dir::Down => {self.head.add((1,0))}
      Dir::Right =>{self.head.add((0,1));}
      Dir::Left =>{self.head.sub((0,1));}
      Dir::Up =>{self.head.sub((1,0));}
      _=> panic!("Cannot have the head not have a direction")
    };
  }
  fn ate(&mut self) {
    self.path.push(self.head);
    self.length += 1;
  }
  fn turn(&mut self, picked_move:Dir) {
    let current_dir = self.dir;
    if picked_move == None {return;}
    let (a,b) = match current_dir {
      Dir::Down => (-1,0),
      Dir::Up => (1,0),
      Dir::Left => (0,-1),
      Dir::Right => (0,1),
      Dir::None => panic!("Cannot have no direction")
    };
    let (c,d) = match picked_move {
      Dir::Down => (-1,0),
      Dir::Up => (1,0),
      Dir::Left => (0,-1),
      Dir::Right => (0,1),
      Dir::None => panic!("Literally should be impossible")
    };
    if (a + c == 0) && (b + d == 0) {return;}
    if ((a + c).abs() == 2) || ((b + d).abs() == 2) {return;}
    self.dir = picked_move;
  }
}

impl SnakeGame {
  fn new(game_size:Option<usize>, level:f32) -> Self {
    Self { 
      current_frame: Matrix::new(game_size.unwrap_or(20), 0.0), 
      apple_gradient: Matrix::new(game_size.unwrap_or(20), 0.0), 
      snake: Snake::new(
        Coords { row: game_size/2, col: game_size/2 }, 
        level,
      ), 
      apple: Self::apple_new(game_size), 
      score: 0, 
      pixle_size: pixle_size_calculator(game_size.unwrap_or(20)),
    }




  }
  fn progress_frame(&mut self)  {
    todo!()
  }
  fn check_edge_collide(&mut self, game_size:usize) {
    // If too far out
    if self.snake.head.is_any_greater_than(game_size, game_size) {
      self.restart_game();
      return;
    }
    // If too far in
    if Some(&self.snake.head) == self.snake.path.last() 
    && self.snake.head.is_any_same(0, 0) 
    {
      self.restart_game();
      return;
    }
    
    todo!()
  }
  fn check_self_collide(&mut self) {
    for i in &self.snake.path {
      if i == &self.snake.head {
        self.restart_game();
        break;
      }
    }
  }
  fn restart_game(&mut self) {
    todo!()
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
    todo!()
  }
  
  fn draw(&self) {
    let length = self.pixle_size;
    for xcell in 0..self.current_frame.rows {
      for ycell in 0..self.current_frame.cols {
        if let Some(value) = self.current_frame.get(xcell, ycell) {
          let color = if *state > 0 {
            WHITE
          } else {
            // needs a color fn that makes a color red->black based on the apple gradient
            todo!()
          };
          draw_rectangle((xcell as f32) * length, (ycell as f32) * length, length, length, color);
        }}}
  }

  fn snake_ate_apple(&mut self) -> bool {
    if !self.check_snake_ate() {return false;}
    self.snake.ate();
    self.change_apple_pos();
    true
  }
  fn apple_new(game_size:Option<usize>) -> Coords {
    todo!()
  }
  fn change_apple_pos(&mut self) {
    // take the current positions of the snake body, and place the apple somewhere random in the extra space
    loop {
      let mut possible = true;
      let cord = Coords {
        row: rand::gen_range(0, self.apple_gradient.rows),
        col: rand::gen_range(0, self.apple_gradient.cols),
      };
      if &cord == &self.apple {continue;}
      for i in &self.snake.path {
        if &cord == i {
          possible = false;
          break;
        }
      }
      if possible {
        self.apple = cord;
        break;
      }
    }
  }
  fn check_snake_ate(&self) -> bool {
    self.apple == self.snake.head
  }


  fn pixle_to_grid(&self, pixle:f32) -> usize {
    (pixle/self.pixle_size).floor() as usize
  }
  fn level_up(&mut self) {
    todo!()
  }
}

fn pixle_size_calculator(game_size:usize) -> f32 {
  if game_size == 0 {panic!("Chosen game size is too large");}
  let smallest = std::cmp::min(screen_height().round() as i32, screen_width().round() as i32);
  if smallest > std::cmp::min(smallest, game_size as i32) {
    return (smallest as f32/game_size as f32) as f32
  } else { panic!("Chosen game size is too large") }
}
fn get_move() -> Dir {
  if is_key_released(KeyCode::Down) {
    return Dir::Down;
  }
  if is_key_released(KeyCode::Up) {
    return Dir::Up;
  }
  if is_key_released(KeyCode::Left) {
    return Dir::Left;
  }
  if is_key_released(KeyCode::Right) {
    return Dir::Right;
  }

  Dir::None
}


