#![allow(non_snake_case)]

mod bbob;
mod eval;
mod matrix;
mod problem;

pub use crate::bbob::{Function, Params, DIMENSIONS};
pub use crate::matrix::InputBatch;
pub use crate::problem::Problem;

pub use strum::IntoEnumIterator;
