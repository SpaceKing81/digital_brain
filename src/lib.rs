pub mod axion;
pub mod brain;
pub mod consts;
pub mod grid;
pub mod input;
pub mod neuron;
pub mod output;

// re-export your main Brain type at the crate root:
pub use brain::Brain;