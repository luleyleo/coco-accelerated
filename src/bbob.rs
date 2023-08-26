#![allow(clippy::needless_range_loop)]

pub static VALID_DIMENSIONS: &[usize] = &[2, 3, 5, 10, 20, 40];

mod params;
use params::Params;

mod futhark;
use futhark::{evaluate_function, FutharkParams};

mod function;
pub use function::Function;

mod problem;
pub use problem::{Evaluator, Problem};
