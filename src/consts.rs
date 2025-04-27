//Simulation
pub const STARTING_NEURONS:u32 = 1000;
pub const STARTING_INPUTS:u32 = 10;
pub const IDEAL_TPS:f64 = 60.0;
pub const GRID_SIZE: f32 = 20.0;

// Physics
pub const GRAVITY: f32 = 0.01;
pub const GRAVITY_SUFRACE: f32 = 150.0;
pub const ELECTRIC_SUFRACE: f32 = 10.0;
pub const COULOMB: f32 = 10.0;  
pub const SPRING:f32 = 0.01;
pub const SPRING_NORMAL:f32 = 11.0;
pub const TIME_STEP: f32 = 0.00005;

// Neurons
pub const MEMORY_SIZE:usize = 5; // Also Cooldown time
pub const ONE_STANDARD_DEV_THRESHOLD:i32 = 30;
pub const MAX_HAPPY_VALUE:u32 = 500; // The value that it dies at
pub const INACTIVITY_DEATH_TIME:u32 = 10000; // The time before it dies of bordom
pub const MAX_THRESHOLD: u32 = 70;
pub const MIN_THRESHOLD: u32 = 30;

// Outputs
pub const OUTPUT_THRESHOLD: i32 = 70;