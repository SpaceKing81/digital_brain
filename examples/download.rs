use macroquad::{prelude::*};
use digital_brain::Spirion;


fn main() {
  let (mut brain,inputs, outputs) = Spirion::spin_up_new(
    Some(10), 
    4, 4,
    false,
  );
  brain.save_as_bin();

}