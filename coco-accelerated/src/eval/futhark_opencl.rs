use crate::{eval::eval_futhark, Context, Function, InputMatrix, Problem};

pub fn futhark_opencl(problem: &Problem, x: InputMatrix) -> Vec<f64> {
    eval_futhark(eval_futhark_opencl, problem, x)
}

fn eval_futhark_opencl(
    ctx: &Context,
    x: InputMatrix,
    xopt: Vec<f64>,
    R: coco_legacy::Matrix,
    Q: coco_legacy::Matrix,
    function: Function,
    fopt: f64,
) -> Option<Vec<f64>> {
    let ctx = &ctx.futhark_opencl;

    eval_futhark_using!(coco_futhark_opencl, ctx, x, xopt, R, Q, function, fopt)
}
