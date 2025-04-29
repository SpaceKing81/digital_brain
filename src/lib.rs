pub mod brain;
mod axion;
mod consts;
mod grid;
mod input;
mod neuron;
mod output;

// re-export your main Brain type at the crate root:
pub use brain::Brain;



#[test]
fn test_placeholder() {
  // assert_eq!(true, false);
  assert_eq!(true, true);
}