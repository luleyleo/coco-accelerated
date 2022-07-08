use coco_accelerated::{Context, Function, Problem, DIMENSIONS};

#[test]
fn can_evaluate_coco() {
    let context = &mut Context::default();
    let problem = Problem::new(context, Function::Sphere);

    for dim in DIMENSIONS {
        let _ = problem.eval_coco_single(&vec![1.0; *dim]);
    }
}

#[test]
fn can_evaluate_futhark() {
    let context = &mut Context::default();

    let problem = Problem::new(context, Function::Sphere);

    for dim in DIMENSIONS {
        let _ = problem.eval_futhark_c_single(&vec![1.0; *dim]);
    }
}