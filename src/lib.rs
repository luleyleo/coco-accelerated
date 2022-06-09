use crate::bbob::Function;

pub mod bbob;
mod eval;

pub struct Problem {
    function: Function,
    instance: usize,
}

impl Problem {
    pub fn new(function: Function) -> Self {
        Problem::new_instance(function, 1)
    }

    pub fn new_instance(function: Function, instance: usize) -> Self {
        Problem { function, instance }
    }

    pub fn eval_coco(&self, x: &[f64]) -> f64 {
        assert!(bbob::DIMENSIONS.contains(&x.len()));

        eval::coco(self, x)
    }

    pub fn eval_accelerated(&self, x: &[f64]) -> f64 {
        eval::accelerated(self, x)
    }
}
