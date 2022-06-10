use coco_accelerated::{
    bbob::{self, Function},
    Context, Problem,
};

#[test]
fn can_evaluate_coco() {
    let context = &mut Context::default();
    let mut problem = Problem::new(context, Function::Sphere);

    for dim in bbob::DIMENSIONS {
        let _ = problem.eval_coco(&vec![1.0; *dim]);
    }
}

#[test]
fn can_evaluate_accelerated() {
    let context = &mut Context::default();
    let mut problem = Problem::new(context, Function::Sphere);

    for dim in bbob::DIMENSIONS {
        let _ = problem.eval_accelerated(&vec![1.0; *dim]);
    }
}
