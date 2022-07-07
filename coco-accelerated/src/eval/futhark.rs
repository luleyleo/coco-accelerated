use crate::{Context, Function, Problem};

pub type EvalFn = fn(
    ctx: &Context,
    x: &[f64],
    xopt: Vec<f64>,
    R: coco_legacy::Matrix,
    Q: coco_legacy::Matrix,
    function: Function,
    fopt: f64,
) -> Option<f64>;

pub fn eval_futhark(eval_fn: EvalFn, problem: &Problem, x: &[f64]) -> f64 {
    let Problem {
        function, instance, ..
    } = *problem;

    let dimension = x.len();
    let rseed: usize = function as usize + 10000 * instance;
    let rseed_3: usize = 3 + 10000 * instance;
    let rseed_17: usize = 17 + 10000 * instance;

    // These (as well as bent_cigar) use a different rseed.
    let rseed = match function {
        Function::BuecheRastrigin => rseed_3,
        Function::Schaffers2 => rseed_17,
        _ => rseed,
    };

    let mut xopt = coco_legacy::compute_xopt(rseed, x.len());
    let fopt = coco_legacy::compute_fopt(function as usize, instance);

    // Special cases for some functions.
    match function {
        Function::BuecheRastrigin => {
            // OME: This step is in the legacy C code but _not_ in the function description.
            for xi in xopt.iter_mut().step_by(2) {
                *xi = xi.abs();
            }
        }
        Function::LinearSlope => {
            for xi in &mut xopt {
                *xi = if *xi >= 0.0 { 5.0 } else { -5.0 };
            }
        }
        Function::Rosenbrock => {
            // According to the documentation, xopt should be within [-3, 3],
            // but this is not enough to satisfy that condition...
            for xi in &mut xopt {
                *xi *= 0.75;
            }
        }
        Function::BentCigar => {
            // No clue why they did this, probably it was a typo?
            xopt = coco_legacy::compute_xopt(rseed + 1000000, x.len());
        }
        Function::Schwefel => {
            xopt = coco_legacy::compute_unif(rseed, x.len());
            for xi in &mut xopt {
                *xi = if *xi >= 0.5 { 1.0 } else { -1.0 };
            }
        }
        _ => {}
    }

    let R = coco_legacy::compute_rotation(rseed + 1000000, dimension);
    let Q = coco_legacy::compute_rotation(rseed, dimension);

    let result = eval_fn(problem.context, x, xopt, R, Q, function, fopt);

    result.unwrap_or_else(|| panic!("Failed to evaluate {:?}", function))
}
