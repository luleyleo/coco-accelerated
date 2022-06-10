use coco_accelerated::{bbob::Function, Problem};
use proptest::{prelude::*, test_runner::FileFailurePersistence};

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

macro_rules! assert_float_eq {
    ($left:expr, $right:expr) => {
        float_eq::assert_float_eq!($left, $right, ulps <= 10)
    };
    ($left:expr, $right:expr, $eq:ident <= $tol:expr) => {
        float_eq::assert_float_eq!($left, $right, ulps <= 10, $eq <= $tol)
    };
}

proptest! {
    #![proptest_config(ProptestConfig {
        fork: true,
        timeout: 1000,
        failure_persistence: Some(Box::new(FileFailurePersistence::WithSource("regressions"))),
        .. ProptestConfig::default()
    })]

    #[test]
    fn sphere(x in x_strategy()) {
        let problem = Problem::new(Function::Sphere);
        let cres = problem.eval_coco(&x);
        let ares = problem.eval_accelerated(&x);

        assert_float_eq!(cres, ares);
    }

    #[test]
    fn ellipsoid(x in x_strategy()) {
        let problem = Problem::new(Function::Ellipsoid);
        let cres = problem.eval_coco(&x);
        let ares = problem.eval_accelerated(&x);

        // TODO: Improve accuracy
        assert_float_eq!(cres, ares,  abs <= 1e-12);
    }

    #[test]
    fn rastrigin(x in x_strategy()) {
        let problem = Problem::new(Function::Rastrigin);
        let cres = problem.eval_coco(&x);
        let ares = problem.eval_accelerated(&x);

        // TODO: Improve accuracy
        assert_float_eq!(cres, ares, abs <= 1e-10);
    }

    #[test]
    fn bueche_rastrigin(x in x_strategy()) {
        let problem = Problem::new(Function::BuecheRastrigin);
        let cres = problem.eval_coco(&x);
        let ares = problem.eval_accelerated(&x);

        assert_float_eq!(cres, ares);
    }

}
