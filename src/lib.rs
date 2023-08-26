#![allow(non_snake_case)]

mod batch;
mod bbob;
mod eval;
mod problem;

pub use crate::batch::InputBatch;
pub use crate::bbob::{Function, Params, DIMENSIONS};
pub use crate::problem::Problem;

pub use strum::IntoEnumIterator;
