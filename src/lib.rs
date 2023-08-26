#![allow(non_snake_case)]

pub mod bbob;

mod batch;
pub use crate::batch::InputBatch;

/// Re-export.
pub use strum::IntoEnumIterator;
