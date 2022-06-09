use crate::{bbob::Function, Problem};
use accelerated::functions;

pub fn coco(problem: &Problem, x: &[f64]) -> f64 {
    let mut suite = coco::Suite::new(coco::SuiteName::Bbob, "", "").unwrap();
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
    let Problem { function, instance } = *problem;

    let ctx = &accelerated::Context::default();

    let rseed: usize = function as usize + 10000 * instance;
    let rseed_3: usize = 3 + 10000 * instance;
    let rseed_17: usize = 17 + 10000 * instance;

    let rseed = match function {
        Function::BuecheRastrigin => rseed_3,
        Function::Schaffers2 => rseed_17,
        _ => rseed,
    };

    let xopt = coco_legacy::compute_xopt(rseed, x.len());
    let fopt = coco_legacy::compute_fopt(function as usize, instance);

    let x = &accelerated::storage::F64_1D::new(ctx, x);
    let xopt = &accelerated::storage::F64_1D::new(ctx, &xopt);

    let result = match function {
        Function::Sphere => functions::sphere_bbob(ctx, x, xopt, fopt),
        Function::Ellipsoid => functions::ellipsoidal_bbob(ctx, x, xopt, fopt),
        Function::Rastrigin => functions::rastrigin_bbob(ctx, x, xopt, fopt),
        Function::BuecheRastrigin => todo!(),
        Function::LinearSlope => todo!(),
        Function::AttractiveSector => todo!(),
        Function::StepEllipsoid => todo!(),
        Function::Rosenbrock => todo!(),
        Function::RosenbrockRotated => todo!(),
        Function::EllipsoidRotated => todo!(),
        Function::Discus => todo!(),
        Function::BentCigar => todo!(),
        Function::SharpRidge => todo!(),
        Function::DifferentPowers => todo!(),
        Function::RastriginRotated => todo!(),
        Function::Weierstrass => todo!(),
        Function::Schaffers1 => todo!(),
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
