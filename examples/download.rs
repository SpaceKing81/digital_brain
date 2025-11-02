use macroquad::{prelude::*};
use digital_brain::Spirion;


fn main() {
  
  let (mut brain,inputs, outputs) = Spirion::spin_up_new(
    Some(10), 
    (GAME_SIZE.unwrap_or(30)*GAME_SIZE.unwrap_or(30)) as u128, 2,
    true,
  );


}