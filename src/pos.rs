use serde::{Serialize, Deserialize};


#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pos {
  pub x:f32,
  pub y:f32,
}

impl Pos {
  pub fn new(x:f32,y:f32) -> Self {
    Pos {
      x,
      y,
    }
  }

  pub fn zero() -> Self {
    Self::new(0.0, 0.0)
  }

}