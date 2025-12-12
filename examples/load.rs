use macroquad::{prelude::*};
use digital_brain::Spirion;


fn main() {
  let mut brain = digital_brain::Spirion::build_from_bin("/spirion.bin");
  brain.save_as_bin();

}