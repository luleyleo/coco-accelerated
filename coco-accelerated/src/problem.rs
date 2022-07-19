use crate::{eval, Context, Function, InputMatrix, DIMENSIONS};

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

    pub fn eval_coco(&self, x: InputMatrix) -> Vec<f64> {
        assert!(DIMENSIONS.contains(&x.dimension()));

        let mut suite = self.context.coco.borrow_mut();
        let mut problem = suite
            .problem_by_function_dimension_instance(
                self.function as usize,
                x.dimension(),
                self.instance,
            )
            .unwrap();

        let mut output = Vec::with_capacity(x.inputs());
        for x in x.iter_inputs() {
            output.push({
                let y = &mut [0.0];
                problem.evaluate_function(x, y);
                y[0]
            });
        }

        output
    }

    #[cfg(feature = "c")]
    pub fn eval_futhark_c(&self, x: InputMatrix) -> Vec<f64> {
        eval::futhark_c(self, x)
    }

    #[cfg(feature = "multicore")]
    pub fn eval_futhark_multicore(&self, x: InputMatrix) -> Vec<f64> {
        eval::futhark_multicore(self, x)
    }

    #[cfg(feature = "opencl")]
    pub fn eval_futhark_opencl(&self, x: InputMatrix) -> Vec<f64> {
        eval::futhark_opencl(self, x)
    }

    #[cfg(feature = "cuda")]
    pub fn eval_futhark_cuda(&self, x: InputMatrix) -> Vec<f64> {
        eval::futhark_cuda(self, x)
    }

    pub fn eval_coco_single(&self, x: &[f64]) -> f64 {
        let x = InputMatrix::new(x, x.len());

        self.eval_coco(x).pop().unwrap()
    }

    #[cfg(feature = "c")]
    pub fn eval_futhark_c_single(&self, x: &[f64]) -> f64 {
        let x = InputMatrix::new(x, x.len());

        self.eval_futhark_c(x).pop().unwrap()
    }
}
