use crate::{eval, Context, Function, DIMENSIONS};

pub struct Problem<'c> {
    pub(crate) context: &'c Context,
    pub(crate) function: Function,
    pub(crate) instance: usize,
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
        assert!(DIMENSIONS.contains(&x.len()));

        eval::coco(self, x)
    }

    #[cfg(feature = "c")]
    pub fn eval_futhark_c(&mut self, x: &[f64]) -> f64 {
        eval::futhark_c(self, x)
    }

    #[cfg(feature = "multicore")]
    pub fn eval_futhark_multicore(&mut self, x: &[f64]) -> f64 {
        eval::futhark_multicore(self, x)
    }

    #[cfg(feature = "opencl")]
    pub fn eval_futhark_opencl(&mut self, x: &[f64]) -> f64 {
        eval::futhark_opencl(self, x)
    }
}
