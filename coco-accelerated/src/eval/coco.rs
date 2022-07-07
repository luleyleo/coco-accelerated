use crate::Problem;

pub fn coco(problem: &Problem, x: &[f64]) -> f64 {
    let mut suite = problem.context.coco.borrow_mut();
    let mut problem = suite
        .problem_by_function_dimension_instance(
            problem.function as usize,
            x.len(),
            problem.instance,
        )
        .unwrap();

    let y = &mut [0.0];
    problem.evaluate_function(x, y);
    y[0]
}
