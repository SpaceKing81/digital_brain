use macroquad::{prelude::*};
use digital_brain::Spirion;


fn main() {
  let (mut brain,inputs, outputs) = Spirion::spin_up_new(
    Some(1000), 
    40, 40,
    false,
  );
  brain.save_as_bin();

}