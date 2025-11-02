use macroquad::{prelude::*};
use digital_brain::Spirion;


fn main() {
  let (mut brain,inputs, outputs) = Spirion::spin_up_new(
    Some(10), 
    2 as u128, 2,
    true,
  );
  brain.save_as_bin();

}