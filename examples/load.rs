use macroquad::{prelude::*};
use digital_brain::Spirion;


fn main() {
  let (mut brain,_,_) = digital_brain::Spirion::build_from_bin(
    "spirion_load_test.bin"
  );
    

  brain.save_as_bin("spirion_load_test.bin");
}