use coco_accelerated::{Context, Function, Problem, DIMENSIONS};

#[test]
fn can_evaluate_coco() {
    let context = &mut Context::default();

    for dim in DIMENSIONS.iter().copied() {
        let mut coco_suite = coco_accelerated::reference::Suite::new();
        let problem = Problem::new(context, Function::Sphere, dim);
        let mut coco_problem = problem.get_reference_instance(&mut coco_suite);
        let _ = coco_problem.eval_coco_single(&vec![1.0; dim]);
    }
}

#[test]
#[allow(unused_mut, unused_variables)]
fn can_evaluate_futhark() {
    let context = &mut Context::default();

    for dim in DIMENSIONS.iter().copied() {
        let mut problem = Problem::new(context, Function::Sphere, dim);

        #[cfg(feature = "c")]
        let _ = problem.eval_futhark_c_single(&vec![1.0; dim]);

        #[cfg(feature = "opencl")]
        let _ = problem.eval_futhark_opencl_single(&vec![1.0; dim]);
    }
}
