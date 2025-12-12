use macroquad::{prelude::*};
use digital_brain::Spirion;


fn main() {
  let mut brain:Spirion = 
    match digital_brain::Spirion::build_from_bin(
      "spirion_load_test.bin"
    ) {
      Ok(data) => {
        data
      },
      Err(erro) => {
        eprintln!(
          "Bad name or incompatable data (not bin, not encoded by bin, etc): {}"
          , erro);
        unreachable!();
      }
    };


  brain.save_as_bin("spirion_load_test.bin");

}