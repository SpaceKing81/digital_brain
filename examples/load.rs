use macroquad::{prelude::*};
use digital_brain::Spirion;


fn main() {
  let mut brain:Spirion = 
    match digital_brain::Spirion::build_from_bin(
      "spirion.bin"
    ) {
      Ok(data) => {
        data
      },
      Err(erro) => {
        eprintln!("Bad name or incompatable: {}", erro);
        unreachable!();
      }
    };


  brain.save_as_bin();

}