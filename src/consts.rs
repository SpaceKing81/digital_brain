use macroquad::color::Color;
//Simulation
pub const GRID_SIZE: f32 = 60.0;

// Physics
pub const GRAVITY: f32 = 0.01;
pub const GRAVITY_SUFRACE: f32 = 20.0;
pub const ELECTRIC_SUFRACE: f32 = 11.0;
pub const COULOMB: f32 = 1000.0;  
pub const SPRING:f32 = 10.0;
pub const SPRING_NORMAL:f32 = 11.0;
pub const TIME_STEP: f32 = 0.01;

// Neurons
pub const MEMORY_SIZE:usize = 5; // Also Cooldown time
pub const ONE_STANDARD_DEV_THRESHOLD:i32 = 30;
pub const MAX_HAPPY_VALUE:u32 = 500; // The value that it dies at
pub const INACTIVITY_DEATH_TIME:u32 = 1000000000; // The time before it dies of bordom
pub const MAX_THRESHOLD: u32 = 70;
pub const MIN_THRESHOLD: u32 = 30;
pub const OUTPUT_COLOR:Color = Color::new(0.0, 0.5, 1.0, 1.0);

// Outputs
pub const AXION_NEG_COLOR:Color = Color::new(0.6, 0.2, 0.0, 1.0);
pub const AXION_POS_COLOR:Color = Color::new(0.1, 0.5, 0.3, 1.0);
pub const AXION_INPUT_COLOR:Color = Color::new(0.9, 0.3, 0.0, 1.0);