use crate::{matrix::InputMatrix, Context, Function, Problem};

pub type EvalFn = fn(
    ctx: &Context,
    x: InputMatrix,
    xopt: Vec<f64>,
    R: coco_legacy::Matrix,
    Q: coco_legacy::Matrix,
    function: Function,
    fopt: f64,
) -> Option<Vec<f64>>;

pub fn eval_futhark(eval_fn: EvalFn, problem: &Problem, x: InputMatrix) -> Vec<f64> {
    let Problem {
        function, instance, ..
    } = *problem;

    let dimension = x.dimension;
    let rseed: usize = function as usize + 10000 * instance;
    let rseed_3: usize = 3 + 10000 * instance;
    let rseed_17: usize = 17 + 10000 * instance;

    // These (as well as bent_cigar) use a different rseed.
    let rseed = match function {
        Function::BuecheRastrigin => rseed_3,
        Function::Schaffers2 => rseed_17,
        _ => rseed,
    };

    let mut xopt = coco_legacy::compute_xopt(rseed, x.dimension);
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
            xopt = coco_legacy::compute_xopt(rseed + 1000000, x.inputs());
        }
        Function::Schwefel => {
            xopt = coco_legacy::compute_unif(rseed, x.inputs());
            for xi in &mut xopt {
                *xi = if *xi >= 0.5 { 1.0 } else { -1.0 };
            }
        }
        _ => {}
    }

    let (nR, nQ) = needs_rotation(function);
    let R = nR
        .then(|| coco_legacy::compute_rotation(rseed + 1000000, dimension))
        .unwrap_or_default();
    let Q = nQ
        .then(|| coco_legacy::compute_rotation(rseed, dimension))
        .unwrap_or_default();

    let result = eval_fn(problem.context, x, xopt, R, Q, function, fopt);

    result.unwrap_or_else(|| panic!("Failed to evaluate {:?}", function))
}

#[macro_export]
macro_rules! eval_futhark_using {
    ($backend:ident, $ctx:ident, $x:ident, $xopt:ident, $R:ident, $Q:ident, $function:ident, $fopt:ident) => {{
        use $backend::{functions, storage};

        let ctx: &$backend::Context = $ctx;
        let x: InputMatrix = $x;
        let xopt: Vec<f64> = $xopt;
        let R: coco_legacy::Matrix = $R;
        let Q: coco_legacy::Matrix = $Q;
        let function: Function = $function;
        let fopt: f64 = $fopt;

        let mut output = Vec::with_capacity(x.inputs());

        let x = &storage::F64_2D::new(ctx, x.data, x.dimension);
        let xopt = &storage::F64_1D::new(ctx, &xopt);
        let R = &storage::F64_2D::new(ctx, &R.data, R.dimension);
        let Q = &storage::F64_2D::new(ctx, &Q.data, Q.dimension);

        let success = match function {
            Function::Sphere => functions::sphere_bbob(ctx, &mut output, x, xopt, fopt),
            Function::Ellipsoid => functions::ellipsoidal_bbob(ctx, &mut output, x, xopt, fopt),
            Function::Rastrigin => functions::rastrigin_bbob(ctx, &mut output, x, xopt, fopt),
            Function::BuecheRastrigin => {
                functions::bueche_rastrigin_bbob(ctx, &mut output, x, xopt, fopt)
            }
            Function::LinearSlope => functions::linear_slope_bbob(ctx, &mut output, x, xopt, fopt),
            Function::AttractiveSector => {
                functions::attractive_sector_bbob(ctx, &mut output, x, xopt, fopt, R, Q)
            }
            Function::StepEllipsoid => {
                functions::step_ellipsoidal_bbob(ctx, &mut output, x, xopt, fopt, R, Q)
            }
            Function::Rosenbrock => functions::rosenbrock_bbob(ctx, &mut output, x, xopt, fopt),
            Function::RosenbrockRotated => {
                functions::rosenbrock_rotated_bbob(ctx, &mut output, x, fopt, Q)
            }
            Function::EllipsoidRotated => {
                functions::ellipsoidal_rotated_bbob(ctx, &mut output, x, xopt, fopt, R)
            }
            Function::Discus => functions::discus_bbob(ctx, &mut output, x, xopt, fopt, R),
            Function::BentCigar => functions::bent_cigar_bbob(ctx, &mut output, x, xopt, fopt, R),
            Function::SharpRidge => {
                functions::sharp_ridge_bbob(ctx, &mut output, x, xopt, fopt, R, Q)
            }
            Function::DifferentPowers => {
                functions::different_powers_bbob(ctx, &mut output, x, xopt, fopt, R)
            }
            Function::RastriginRotated => {
                functions::rastrigin_rotated_bbob(ctx, &mut output, x, xopt, fopt, R, Q)
            }
            Function::Weierstrass => {
                functions::weierstrass_bbob(ctx, &mut output, x, xopt, fopt, R, Q)
            }
            Function::Schaffers1 => {
                functions::schaffers_f7_bbob(ctx, &mut output, x, xopt, fopt, R, Q)
            }
            Function::Schaffers2 => {
                functions::schaffers_f7_ill_bbob(ctx, &mut output, x, xopt, fopt, R, Q)
            }
            Function::GriewankRosenbrock => {
                functions::griewank_rosenbrock_bbob(ctx, &mut output, x, fopt, Q)
            }
            Function::Schwefel => functions::schwefel_bbob(ctx, &mut output, x, xopt, fopt),
            Function::Gallagher1 => todo!(),
            Function::Gallagher2 => todo!(),
            Function::Katsuura => todo!(),
            Function::LunacekBiRastrigin => todo!(),
        };

        success.then(|| output)
    }};
}

fn needs_rotation(function: Function) -> (bool, bool) {
    match function {
        Function::Sphere => (false, false),
        Function::Ellipsoid => (false, false),
        Function::Rastrigin => (false, false),
        Function::BuecheRastrigin => (false, false),
        Function::LinearSlope => (false, false),
        Function::AttractiveSector => (true, true),
        Function::StepEllipsoid => (true, true),
        Function::Rosenbrock => (false, false),
        Function::RosenbrockRotated => (false, true),
        Function::EllipsoidRotated => (true, false),
        Function::Discus => (true, false),
        Function::BentCigar => (true, false),
        Function::SharpRidge => (true, true),
        Function::DifferentPowers => (true, false),
        Function::RastriginRotated => (true, true),
        Function::Weierstrass => (true, true),
        Function::Schaffers1 => (true, true),
        Function::Schaffers2 => (true, true),
        Function::GriewankRosenbrock => (false, true),
        Function::Schwefel => (false, false),
        Function::Gallagher1 => todo!(),
        Function::Gallagher2 => todo!(),
        Function::Katsuura => todo!(),
        Function::LunacekBiRastrigin => todo!(),
    }
}
