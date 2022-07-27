#![allow(non_snake_case)]

mod bbob;
mod context;
mod eval;
mod matrix;
mod problem;

pub use crate::bbob::{Function, Params, DIMENSIONS};
pub use crate::context::Context;
pub use crate::matrix::InputMatrix;
pub use crate::problem::Problem;

pub use strum::IntoEnumIterator;
