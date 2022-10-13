use std::marker::PhantomData;

use crate::{eval, Context, Function, InputMatrix, Params};

pub struct Problem<'c> {
    pub function: Function,
    pub dimension: usize,
    pub instance: usize,

    #[cfg(feature = "c")]
    pub(crate) instance_c: eval::futhark_c::Problem<'c>,

    #[cfg(feature = "multicore")]
    pub(crate) instance_multicore: eval::futhark_multicore::Problem<'c>,

    #[cfg(feature = "opencl")]
    pub(crate) instance_opencl: eval::futhark_opencl::Problem<'c>,

    #[cfg(feature = "cuda")]
    pub(crate) instance_cuda: eval::futhark_cuda::Problem<'c>,

    phantom: PhantomData<&'c ()>,
}

impl<'c> Problem<'c> {
    pub fn new(context: &'c Context, function: Function, dimension: usize) -> Self {
        Problem::new_instance(context, function, dimension, 1)
    }

    pub fn new_instance(
        context: &'c Context,
        function: Function,
        dimension: usize,
        instance: usize,
    ) -> Self {
        let params = Params::from(function, dimension, instance);

        #[cfg(feature = "c")]
        let instance_c = eval::futhark_c::Problem::new(
            &context.coco_futhark_c,
            function,
            eval::futhark_c::FParams::from(&context.coco_futhark_c, &params),
        );

        #[cfg(feature = "multicore")]
        let instance_multicore = eval::futhark_multicore::Problem::new(
            &context.coco_futhark_multicore,
            function,
            eval::futhark_multicore::FParams::from(&context.coco_futhark_multicore, &params),
        );

        #[cfg(feature = "opencl")]
        let instance_opencl = eval::futhark_opencl::Problem::new(
            &context.coco_futhark_opencl,
            function,
            eval::futhark_opencl::FParams::from(&context.coco_futhark_opencl, &params),
        );

        #[cfg(feature = "cuda")]
        let instance_cuda = eval::futhark_cuda::Problem::new(
            &context.coco_futhark_cuda,
            function,
            eval::futhark_cuda::FParams::from(&context.coco_futhark_cuda, &params),
        );

        Problem {
            function,
            dimension,
            instance,

            #[cfg(feature = "c")]
            instance_c,

            #[cfg(feature = "multicore")]
            instance_multicore,

            #[cfg(feature = "opencl")]
            instance_opencl,

            #[cfg(feature = "cuda")]
            instance_cuda,

            phantom: PhantomData,
        }
    }

    #[cfg(feature = "reference")]
    pub fn get_reference_instance<'s>(
        &self,
        coco: &'s mut crate::reference::Suite,
    ) -> crate::reference::Problem<'s> {
        let inner = coco
            .inner
            .problem_by_function_dimension_instance(
                self.function as usize,
                self.dimension,
                self.instance,
            )
            .unwrap();

        crate::reference::Problem { inner }
    }

    #[allow(unused_variables, unreachable_code)]
    #[cfg(any(feature = "c", feature = "multicore", feature = "cuda"))]
    pub fn target_hit(&self, value: f64) -> bool {
        #[cfg(feature = "c")]
        return self.instance_c.target_hit(value);

        #[cfg(feature = "multicore")]
        return self.instance_multicore.target_hit(value);

        #[cfg(feature = "cuda")]
        return self.instance_cuda.target_hit(value);
    }

    #[cfg(feature = "c")]
    pub fn eval_futhark_c(&self, x: InputMatrix) -> Vec<f64> {
        assert_eq!(self.dimension, x.dimension());

        self.instance_c.evaluate(x).unwrap()
    }

    #[cfg(feature = "multicore")]
    pub fn eval_futhark_multicore(&self, x: InputMatrix) -> Vec<f64> {
        assert_eq!(self.dimension, x.dimension());

        self.instance_multicore.evaluate(x).unwrap()
    }

    #[cfg(feature = "opencl")]
    pub fn eval_futhark_opencl(&self, x: InputMatrix) -> Vec<f64> {
        assert_eq!(self.dimension, x.dimension());

        self.instance_opencl.evaluate(x).unwrap()
    }

    #[cfg(feature = "cuda")]
    pub fn eval_futhark_cuda(&self, x: InputMatrix) -> Vec<f64> {
        assert_eq!(self.dimension, x.dimension());

        self.instance_cuda.evaluate(x).unwrap()
    }

    #[cfg(feature = "c")]
    pub fn eval_futhark_c_single(&self, x: &[f64]) -> f64 {
        let x = InputMatrix::new(x, x.len());

        self.eval_futhark_c(x).pop().unwrap()
    }

    #[cfg(feature = "opencl")]
    pub fn eval_futhark_opencl_single(&self, x: &[f64]) -> f64 {
        let x = InputMatrix::new(x, x.len());

        self.eval_futhark_opencl(x).pop().unwrap()
    }
}
