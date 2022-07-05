use crate::{bbob::Function, Problem};
use accelerated::functions;

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

pub fn accelerated(problem: &Problem, x: &[f64]) -> f64 {
    let Problem {
        function, instance, ..
    } = *problem;

    let ctx = &problem.context.futhark;

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
        _ => {}
    }

    let x = &accelerated::storage::F64_1D::new(ctx, x);
    let xopt = &accelerated::storage::F64_1D::new(ctx, &xopt);

    let result = match function {
        Function::Sphere => functions::sphere_bbob(ctx, x, xopt, fopt),
        Function::Ellipsoid => functions::ellipsoidal_bbob(ctx, x, xopt, fopt),
        Function::Rastrigin => functions::rastrigin_bbob(ctx, x, xopt, fopt),
        Function::BuecheRastrigin => functions::bueche_rastrigin_bbob(ctx, x, xopt, fopt),
        Function::LinearSlope => functions::linear_slope_bbob(ctx, x, xopt, fopt),
        Function::AttractiveSector => {
            let R = coco_legacy::compute_rotation(rseed + 1000000, dimension);
            let R = &accelerated::storage::F64_2D::new(ctx, &R.data, R.dimension);
            let Q = coco_legacy::compute_rotation(rseed, dimension);
            let Q = &accelerated::storage::F64_2D::new(ctx, &Q.data, Q.dimension);

            functions::attractive_sector_bbob(ctx, x, xopt, fopt, R, Q)
        }
        Function::StepEllipsoid => {
            let R = coco_legacy::compute_rotation(rseed + 1000000, dimension);
            let R = &accelerated::storage::F64_2D::new(ctx, &R.data, R.dimension);
            let Q = coco_legacy::compute_rotation(rseed, dimension);
            let Q = &accelerated::storage::F64_2D::new(ctx, &Q.data, Q.dimension);

            functions::step_ellipsoidal_bbob(ctx, x, xopt, fopt, R, Q)
        }
        Function::Rosenbrock => functions::rosenbrock_bbob(ctx, x, xopt, fopt),
        Function::RosenbrockRotated => {
            let R = coco_legacy::compute_rotation(rseed, dimension);
            let R = &accelerated::storage::F64_2D::new(ctx, &R.data, R.dimension);

            functions::rosenbrock_rotated_bbob(ctx, x, R, fopt)
        }
        Function::EllipsoidRotated => {
            let R = coco_legacy::compute_rotation(rseed + 1000000, dimension);
            let R = &accelerated::storage::F64_2D::new(ctx, &R.data, R.dimension);

            functions::ellipsoidal_rotated_bbob(ctx, x, xopt, fopt, R)
        }
        Function::Discus => {
            let R = coco_legacy::compute_rotation(rseed + 1000000, dimension);
            let R = &accelerated::storage::F64_2D::new(ctx, &R.data, R.dimension);

            functions::discus_bbob(ctx, x, xopt, fopt, R)
        }
        Function::BentCigar => {
            let R = coco_legacy::compute_rotation(rseed + 1000000, dimension);
            let R = &accelerated::storage::F64_2D::new(ctx, &R.data, R.dimension);

            functions::bent_cigar_bbob(ctx, x, xopt, fopt, R)
        }
        Function::SharpRidge => {
            let R = coco_legacy::compute_rotation(rseed + 1000000, dimension);
            let R = &accelerated::storage::F64_2D::new(ctx, &R.data, R.dimension);
            let Q = coco_legacy::compute_rotation(rseed, dimension);
            let Q = &accelerated::storage::F64_2D::new(ctx, &Q.data, Q.dimension);

            functions::sharp_ridge_bbob(ctx, x, xopt, fopt, R, Q)
        }
        Function::DifferentPowers => {
            let R = coco_legacy::compute_rotation(rseed + 1000000, dimension);
            let R = &accelerated::storage::F64_2D::new(ctx, &R.data, R.dimension);

            functions::different_powers_bbob(ctx, x, xopt, fopt, R)
        }
        Function::RastriginRotated => {
            let R = coco_legacy::compute_rotation(rseed + 1000000, dimension);
            let R = &accelerated::storage::F64_2D::new(ctx, &R.data, R.dimension);
            let Q = coco_legacy::compute_rotation(rseed, dimension);
            let Q = &accelerated::storage::F64_2D::new(ctx, &Q.data, Q.dimension);

            functions::rastrigin_rotated_bbob(ctx, x, xopt, fopt, R, Q)
        }
        Function::Weierstrass => {
            let R = coco_legacy::compute_rotation(rseed + 1000000, dimension);
            let R = &accelerated::storage::F64_2D::new(ctx, &R.data, R.dimension);
            let Q = coco_legacy::compute_rotation(rseed, dimension);
            let Q = &accelerated::storage::F64_2D::new(ctx, &Q.data, Q.dimension);

            functions::weierstrass_bbob(ctx, x, xopt, fopt, R, Q)
        }
        Function::Schaffers1 => {
            let R = coco_legacy::compute_rotation(rseed + 1000000, dimension);
            let R = &accelerated::storage::F64_2D::new(ctx, &R.data, R.dimension);
            let Q = coco_legacy::compute_rotation(rseed, dimension);
            let Q = &accelerated::storage::F64_2D::new(ctx, &Q.data, Q.dimension);

            functions::schaffers_f7_bbob(ctx, x, xopt, fopt, R, Q)
        }
        Function::Schaffers2 => todo!(),
        Function::GriewankRosenbrock => todo!(),
        Function::Schwefel => todo!(),
        Function::Gallagher1 => todo!(),
        Function::Gallagher2 => todo!(),
        Function::Katsuura => todo!(),
        Function::LunacekBiRastrigin => todo!(),
    };

    result.unwrap_or_else(|| panic!("Failed to evaluate {:?}", function))
}
