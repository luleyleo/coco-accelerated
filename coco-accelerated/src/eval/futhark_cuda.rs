use crate::{eval::eval_futhark, Context, Function, InputMatrix, Problem};

pub fn futhark_cuda(problem: &Problem, x: InputMatrix) -> Vec<f64> {
    eval_futhark(eval_futhark_cuda, problem, x)
}

fn eval_futhark_cuda(
    ctx: &Context,
    x: InputMatrix,
    xopt: Vec<f64>,
    R: coco_legacy::Matrix,
    Q: coco_legacy::Matrix,
    function: Function,
    fopt: f64,
) -> Option<Vec<f64>> {
    let ctx = &ctx.futhark_cuda;

    eval_futhark_using!(coco_futhark_cuda, ctx, x, xopt, R, Q, function, fopt)
}
