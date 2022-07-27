use crate::{eval, Context, Function, InputMatrix, Params, DIMENSIONS};

pub struct Problem<'c> {
    pub(crate) dimension: usize,

    pub(crate) instance_reference: coco::Problem<'c>,

    #[cfg(feature = "c")]
    pub(crate) instance_c: eval::futhark_c::Problem<'c>,

    #[cfg(feature = "multicore")]
    pub(crate) instance_multicore: eval::futhark_multicore::Problem<'c>,

    #[cfg(feature = "opencl")]
    pub(crate) instance_opencl: eval::futhark_opencl::Problem<'c>,

    #[cfg(feature = "cuda")]
    pub(crate) instance_cuda: eval::futhark_cuda::Problem<'c>,
}

impl<'c> Problem<'c> {
    pub fn new(context: &'c mut Context, function: Function, dimension: usize) -> Self {
        Problem::new_instance(context, function, dimension, 1)
    }

    pub fn new_instance(
        context: &'c mut Context,
        function: Function,
        dimension: usize,
        instance: usize,
    ) -> Self {
        let params = Params::from(function, dimension, instance);

        let instance_reference = context
            .coco
            .problem_by_function_dimension_instance(function as usize, dimension, instance)
            .unwrap();

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
            dimension,
            instance_reference,

            #[cfg(feature = "c")]
            instance_c,

            #[cfg(feature = "multicore")]
            instance_multicore,

            #[cfg(feature = "opencl")]
            instance_opencl,

            #[cfg(feature = "cuda")]
            instance_cuda,
        }
    }

    pub fn eval_coco(&mut self, x: InputMatrix) -> Vec<f64> {
        assert!(DIMENSIONS.contains(&x.dimension()));
        assert_eq!(self.dimension, x.dimension());

        let mut output = Vec::with_capacity(x.inputs());
        for x in x.iter_inputs() {
            output.push({
                let y = &mut [0.0];
                self.instance_reference.evaluate_function(x, y);
                y[0]
            });
        }

        output
    }

    #[cfg(feature = "c")]
    pub fn eval_futhark_c(&mut self, x: InputMatrix) -> Vec<f64> {
        assert_eq!(self.dimension, x.dimension());

        self.instance_c.evaluate(x).unwrap()
    }

    #[cfg(feature = "multicore")]
    pub fn eval_futhark_multicore(&mut self, x: InputMatrix) -> Vec<f64> {
        assert_eq!(self.dimension, x.dimension());

        self.instance_multicore.evaluate(x).unwrap()
    }

    #[cfg(feature = "opencl")]
    pub fn eval_futhark_opencl(&mut self, x: InputMatrix) -> Vec<f64> {
        assert_eq!(self.dimension, x.dimension());

        self.instance_opencl.evaluate(x).unwrap()
    }

    #[cfg(feature = "cuda")]
    pub fn eval_futhark_cuda(&mut self, x: InputMatrix) -> Vec<f64> {
        assert_eq!(self.dimension, x.dimension());

        self.instance_cuda.evaluate(x).unwrap()
    }

    pub fn eval_coco_single(&mut self, x: &[f64]) -> f64 {
        let x = InputMatrix::new(x, x.len());

        self.eval_coco(x).pop().unwrap()
    }

    #[cfg(feature = "c")]
    pub fn eval_futhark_c_single(&mut self, x: &[f64]) -> f64 {
        let x = InputMatrix::new(x, x.len());

        self.eval_futhark_c(x).pop().unwrap()
    }
}
