use crate::InputMatrix;

pub struct Suite {
    pub(crate) inner: coco::Suite,
}

impl Suite {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Suite {
        Suite {
            inner: coco::Suite::new(coco::SuiteName::Bbob, "", "").unwrap(),
        }
    }
}

pub struct Problem<'s> {
    pub(crate) inner: coco::Problem<'s>,
}

impl Problem<'_> {
    pub fn eval_coco(&mut self, x: InputMatrix) -> Vec<f64> {
        assert!(crate::DIMENSIONS.contains(&x.dimension()));
        assert_eq!(self.inner.dimension(), x.dimension());

        let mut output = Vec::with_capacity(x.inputs());
        for x in x.iter_inputs() {
            output.push({
                let y = &mut [0.0];
                self.inner.evaluate_function(x, y);
                y[0]
            });
        }

        output
    }

    pub fn eval_coco_single(&mut self, x: &[f64]) -> f64 {
        let x = InputMatrix::new(x, x.len());

        self.eval_coco(x).pop().unwrap()
    }
}
