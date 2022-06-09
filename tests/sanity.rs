use coco_accelerated::{
    bbob::{self, Function},
    Problem,
};

#[test]
fn can_evaluate_coco() {
    let problem = Problem::new(Function::Sphere);

    for dim in bbob::DIMENSIONS {
        let _ = problem.eval_coco(&vec![1.0; *dim]);
    }
}

#[test]
fn can_evaluate_accelerated() {
    let problem = Problem::new(Function::Sphere);

    for dim in bbob::DIMENSIONS {
        let _ = problem.eval_accelerated(&vec![1.0; *dim]);
    }
}
