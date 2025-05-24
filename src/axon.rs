// use crate::neuron::Neuron;
use macroquad::{rand, color::{Color,GRAY}};

use crate::{internal_consts::{AXON_INPUT_COLOR, AXON_NEG_COLOR, AXON_POS_COLOR}, MAX_HAPPY_VALUE};

#[derive(Clone, Debug, PartialEq)]
pub struct Axon {
  pub id:u128, // personal id
  pub id_source:u32, // id of the signal activator
  pub id_sink:u32, // id of the signal recipient

  is_input:bool,
  pub strength:i32, // how strong the fire is, -100 - 100
  reliable:u32, // value 0-100 in terms of how predictible it is for it to fire, bigger is better
  happyness:u32, // how happy the input neuron (sink) is, 0-150, low is good

  pub delta_t: u32,
  pub avg_t: u32, // average rate of firing
}
// General
impl Axon {
  pub fn new(id_source:u32, id_sink:u32, id:u128, is_input:bool) -> Self {
    Axon {
      id, // personal id
      id_source, // id of the signal activator
      id_sink, // id of the signal recipient
    
      is_input,
      strength:rand::gen_range(-100,100), // how strong the fire is, -100 - 100
      reliable:rand::gen_range(0,50), // value 0-50 in terms of how predictible it is for it to fire, half its max
      happyness:rand::gen_range(0,75), // how happy it is, half its max of 150
    
      delta_t:0,
      avg_t:1,
    }
  }
  
  pub fn update_happyness(&mut self, happyness:u32) {
    self.happyness = happyness;
  }
  
  // takes its self and outputs where its going, and the stength of firing
  // delta_t here is the time since last firing of the neuron
  pub fn fire_axon(&mut self, delta_t:u32) -> (u32,i32) {
    // Order important, don't mix them up
    if self.is_input {
      return (self.id_sink, self.strength);
    }
    self.mutate_strength();
    self.math(delta_t);
    (self.id_sink, self.strength)
  }
  pub fn fire_input(&mut self, delta_t:u32) {
    self.math(delta_t);
  }
  pub fn get_to_draw(&self) -> (u32, u32, Color) {
    if self.is_input {
      return (0, self.id_sink, AXON_INPUT_COLOR);
    }
    
    let (source, sink) = (
      self.id_source, self.id_sink
    );
    let color = match self.strength {
      s if s > 0 => AXON_POS_COLOR, // Green for excitatory
      s if s < 0 => AXON_NEG_COLOR, // Red for inhibitory
      _ => GRAY, // Gray for TB Killed
    };
    (source, sink, color)
  }

  fn math(&mut self, delta_t:u32) {
    self.delta_t = delta_t;
    let t = self.delta_t as i32;
    let w = self.avg_t as i32;
    let delta = (w - t).abs();
    let weight = (delta/25).abs() as u32;

    // alters the reliablity by the distence from average
    if weight == 0 {
      self.reliable += 5;
    } else {
      // 4 is super bad, 3 is bad, 2 is iffy, 1 is same, 0 is perfect
      let weight = weight - 1;
      self.reliable = self.reliable.saturating_sub(weight);
    }
    // sets the average
    self.avg_t = (self.avg_t + self.avg_t + self.delta_t) / 3;
    // Don't need to set delta_t to 0, irrelevent as axons 
    // dont get updated unless being fired
  }
}

// Mutate
impl Axon {
  /// Changes the strength based on how happy and reliable the axon is
  fn mutate_strength(&mut self) {

    let happyness = self.happyness as i32;
    let reliability = self.reliable as i32;

    // the lower the happyness, the less likely to change signs
    let sign = {
      let m = rand::gen_range(-happyness, MAX_HAPPY_VALUE as i32);
      if m == 0 { 1 } else {m/m.abs()}
    };

    // higher reliability makes it more likely to be go farther from 0
    let p = rand::gen_range(-25, reliability);
    let r = rand::gen_range(0, 20) * if p == 0 { 1 } else {p/p.abs()};

    self.strength = if self.strength.is_positive() {self.strength + r} else {self.strength - r};
    self.strength *= sign;
  }
}