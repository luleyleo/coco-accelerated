#![allow(clippy::type_complexity)]

use coco_accelerated::{reference, Context, Function, Problem};
use proptest::{
    prelude::*,
    test_runner::{FileFailurePersistence, TestError, TestRunner},
};

macro_rules! assert_float_eq {
    ($left:expr, $right:expr) => {
        float_eq::assert_float_eq!($left, $right, ulps <= 10)
    };
    ($left:expr, $right:expr, $eq:ident <= $tol:expr) => {
        float_eq::assert_float_eq!($left, $right, ulps <= 10, $eq <= $tol)
    };
}

#[track_caller]
fn strict(c: f64, a: f64) {
    assert_float_eq!(c, a);
}

static FUNCTIONS: &[(Function, fn(f64, f64))] = &[
    (Function::Sphere, strict),
    (Function::Ellipsoid, |c, a| {
        assert_float_eq!(c, a, abs <= 1e-12);
    }),
    (Function::Rastrigin, |c, a| {
        assert_float_eq!(c, a, abs <= 1e-10);
    }),
    (Function::BuecheRastrigin, strict),
    (Function::LinearSlope, strict),
    (Function::AttractiveSector, |c, a| {
        assert_float_eq!(c, a, abs <= 1e-9);
    }),
    (Function::StepEllipsoid, strict),
    (Function::Rosenbrock, strict),
    (Function::RosenbrockRotated, strict),
    (Function::EllipsoidRotated, strict),
    (Function::Discus, strict),
    (Function::BentCigar, strict),
    (Function::SharpRidge, strict),
    (Function::DifferentPowers, strict),
    (Function::RastriginRotated, strict),
    (Function::Weierstrass, |c, a| {
        assert_float_eq!(c, a, abs <= 1e-11);
    }),
    (Function::Schaffers1, |c, a| {
        assert_float_eq!(c, a, abs <= 1e-13);
    }),
    (Function::Schaffers2, |c, a| {
        assert_float_eq!(c, a, abs <= 1e-11);
    }),
    (Function::GriewankRosenbrock, |c, a| {
        assert_float_eq!(c, a, abs <= 1e-11);
    }),
    (Function::Schwefel, strict),
    (Function::Gallagher1, strict),
    (Function::Gallagher2, strict),
    (Function::Katsuura, |c, a| {
        assert_float_eq!(c, a, abs <= 1e-11);
    }),
    (Function::LunacekBiRastrigin, |c, a| {
        assert_float_eq!(c, a, abs <= 1e-12);
    }),
];

fn x_strategy() -> impl Strategy<Value = Vec<f64>> {
    prop_oneof![
        3 => prop::collection::vec(-5.0..=5.0, 2),
        3 => prop::collection::vec(-5.0..=5.0, 3),
        3 => prop::collection::vec(-5.0..=5.0, 5),
        2 => prop::collection::vec(-5.0..=5.0, 10),
        1 => prop::collection::vec(-5.0..=5.0, 20),
        1 => prop::collection::vec(-5.0..=5.0, 40),
    ]
}

#[test]
fn bbob() {
    let config = ProptestConfig {
        source_file: Some(file!()),
        failure_persistence: Some(Box::new(FileFailurePersistence::WithSource("regressions"))),
        ..ProptestConfig::default()
    };

    let strategy = &x_strategy();
    let runner = &mut TestRunner::new(config);
    let mut failed = false;

    for &(function, check) in FUNCTIONS {
        let result = runner.run(strategy, |x| {
            let coco = &mut reference::Suite::new();
            let context = &mut Context::new();
            let problem = &mut Problem::new(context, function, x.len());
            let reference = &mut problem.get_reference_instance(coco);

            let cres = reference.eval_coco_single(&x);
            let ares = problem.eval_futhark_c_single(&x);

            check(cres, ares);

            Ok(())
        });

        failed |= handle_error(function, result);
    }

    assert!(!failed, "Test Failed.");
}

fn handle_error(function: Function, result: Result<(), TestError<Vec<f64>>>) -> bool {
    match &result {
        Ok(_) => println!("{:?} passes.", function),
        Err(TestError::Abort(reason)) => eprintln!("{:?} aborted: {}", function, reason.message()),
        Err(TestError::Fail(reason, input)) => eprintln!(
            "{:?} failed.\n\tinput: {:?}\n\tmessage: {}",
            function,
            input,
            reason.message()
        ),
    }
    result.is_err()
}
