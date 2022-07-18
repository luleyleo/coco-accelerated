use crate::{
    eval::eval_futhark, eval_futhark_using, matrix::InputMatrix, Context, Function, Problem,
};

pub fn futhark_c(problem: &Problem, x: InputMatrix) -> Vec<f64> {
    eval_futhark(eval_futhark_c, problem, x)
}

fn eval_futhark_c(
    ctx: &Context,
    x: InputMatrix,
    xopt: Vec<f64>,
    R: coco_legacy::Matrix,
    Q: coco_legacy::Matrix,
    function: Function,
    fopt: f64,
) -> Option<Vec<f64>> {
    let ctx = &ctx.futhark_c;

    eval_futhark_using!(coco_futhark_c, ctx, x, xopt, R, Q, function, fopt)
}
