#![allow(non_snake_case)]

use std::cell::RefCell;

mod bbob;
mod eval;

pub use crate::bbob::{Function, DIMENSIONS};

pub struct Context {
    pub coco: RefCell<coco::Suite>,
    pub futhark: coco_futhark::Context,
}

impl Context {
    pub fn new() -> Self {
        let coco = RefCell::new(coco::Suite::new(coco::SuiteName::Bbob, "", "").unwrap());
        let futhark = coco_futhark::Context::default();
        Context { coco, futhark }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Problem<'c> {
    context: &'c Context,
    function: Function,
    instance: usize,
}

impl<'c> Problem<'c> {
    pub fn new(context: &'c Context, function: Function) -> Self {
        Problem::new_instance(context, function, 1)
    }

    pub fn new_instance(context: &'c Context, function: Function, instance: usize) -> Self {
        Problem {
            context,
            function,
            instance,
        }
    }

    pub fn eval_coco(&mut self, x: &[f64]) -> f64 {
        assert!(bbob::DIMENSIONS.contains(&x.len()));

        eval::coco(self, x)
    }

    pub fn eval_accelerated(&mut self, x: &[f64]) -> f64 {
        eval::accelerated(self, x)
    }

    pub fn eval_available(&mut self, x: &[f64]) -> f64 {
        match self.function {
            Function::Weierstrass
            | Function::Schwefel
            | Function::Gallagher1
            | Function::Gallagher2
            | Function::Katsuura
            | Function::LunacekBiRastrigin => eval::coco(self, x),
            _ => eval::accelerated(self, x),
        }
    }
}
