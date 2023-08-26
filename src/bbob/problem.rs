use coco_futhark::{backends::Backend, Context};

use crate::{
    bbob::{evaluate_function, Function, FutharkParams, Params},
    InputBatch,
};

#[derive(Debug, Clone)]
pub struct Problem {
    function: Function,
    dimension: usize,
    instance: usize,
    params: Params,
}

impl Problem {
    pub fn new(function: Function, dimension: usize, instance: usize) -> Self {
        let params = Params::new(function, dimension, instance);

        Problem {
            function,
            dimension,
            instance,
            params,
        }
    }

    pub fn function(&self) -> Function {
        self.function
    }

    pub fn dimension(&self) -> usize {
        self.dimension
    }

    pub fn instance(&self) -> usize {
        self.instance
    }

    pub fn fopt(&self) -> f64 {
        self.params.fopt()
    }

    pub fn evaluator<'c, B>(&self, context: &'c Context<B>) -> Evaluator<'c, B>
    where
        B: Backend,
    {
        let function = self.function;
        let futhark_params = FutharkParams::new(context, &self.params);

        Evaluator {
            context,
            function,
            futhark_params,
        }
    }
}

impl PartialEq for Problem {
    fn eq(&self, other: &Self) -> bool {
        self.function == other.function
            && self.dimension == other.dimension
            && self.instance == other.instance
    }
}

impl Eq for Problem {}

pub struct Evaluator<'c, B: Backend> {
    context: &'c Context<B>,
    function: Function,
    futhark_params: FutharkParams<'c, B>,
}

impl<'c, B: Backend> Evaluator<'c, B> {
    pub fn evaluate(&self, x: InputBatch) -> Vec<f64> {
        evaluate_function(self.context, self.function, &self.futhark_params, x).unwrap()
    }

    pub fn evalutate_iter<'a>(
        &self,
        x: impl ExactSizeIterator<Item = impl AsRef<&'a [f64]>>,
    ) -> Vec<f64> {
        let inputs = x.len();

        if inputs == 0 {
            return Vec::new();
        }

        let data = x
            .map(|x| x.as_ref().iter().copied())
            .flatten()
            .collect::<Vec<f64>>();

        let dimension = data.len() / inputs;

        let input_batch = InputBatch::new(&data, dimension);

        self.evaluate(input_batch)
    }
}
