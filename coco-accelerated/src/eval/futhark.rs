#[macro_export]
macro_rules! declare_params {
    ($backend:ident) => {
        pub enum FParams<'c> {
            Basic {
                fopt: f64,
                xopt: $backend::storage::F64_1D<'c>,
            },
            Rotated {
                fopt: f64,
                xopt: $backend::storage::F64_1D<'c>,
                R: $backend::storage::F64_2D<'c>,
            },
            FixedRotated {
                fopt: f64,
                R: $backend::storage::F64_2D<'c>,
            },
            DoubleRotated {
                fopt: f64,
                xopt: $backend::storage::F64_1D<'c>,
                R: $backend::storage::F64_2D<'c>,
                Q: $backend::storage::F64_2D<'c>,
            },
            Gallagher {
                fopt: f64,
                y: $backend::storage::F64_2D<'c>,
                a: $backend::storage::F64_1D<'c>,
                R: $backend::storage::F64_2D<'c>,
            },
        }

        impl<'c> FParams<'c> {
            pub fn from<'a>(ctx: &'c $backend::Context, params: &$crate::Params) -> Self {
                use $backend::storage;
                use $crate::Params;

                match params {
                    &Params::Basic { fopt, ref xopt } => {
                        let xopt = storage::F64_1D::new(ctx, &xopt);
                        FParams::Basic { fopt, xopt }
                    }
                    &Params::Rotated {
                        fopt,
                        ref xopt,
                        ref R,
                    } => {
                        let xopt = storage::F64_1D::new(ctx, &xopt);
                        let R = storage::F64_2D::new(ctx, &R.data, R.dimension, R.dimension);
                        FParams::Rotated { fopt, xopt, R }
                    }
                    &Params::FixedRotated { fopt, ref R } => {
                        let R = storage::F64_2D::new(ctx, &R.data, R.dimension, R.dimension);
                        FParams::FixedRotated { fopt, R }
                    }
                    &Params::DoubleRotated {
                        fopt,
                        ref xopt,
                        ref R,
                        ref Q,
                    } => {
                        let xopt = storage::F64_1D::new(ctx, &xopt);
                        let R = storage::F64_2D::new(ctx, &R.data, R.dimension, R.dimension);
                        let Q = storage::F64_2D::new(ctx, &Q.data, Q.dimension, Q.dimension);
                        FParams::DoubleRotated { fopt, xopt, R, Q }
                    }
                    &Params::Gallagher {
                        fopt,
                        peaks,
                        ref y,
                        ref a,
                        ref R,
                    } => {
                        assert!(peaks == 101 || peaks == 21);
                        assert_eq!(peaks * a.len(), y.len());
                        assert_eq!(a.len(), R.dimension);

                        let y = storage::F64_2D::new(ctx, &y, peaks, a.len());
                        let a = storage::F64_1D::new(ctx, &a);
                        let R = storage::F64_2D::new(ctx, &R.data, R.dimension, R.dimension);

                        FParams::Gallagher { fopt, y, a, R }
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! declare_eval {
    ($backend:ident) => {
        fn eval(
            ctx: &$backend::Context,
            function: $crate::Function,
            params: &FParams,
            x: $crate::InputMatrix,
        ) -> Option<Vec<f64>> {
            use $backend::{functions, storage};
            use $crate::Function;

            let mut output = Vec::with_capacity(x.inputs());
            let x = &storage::F64_2D::new(ctx, x.data(), x.inputs(), x.dimension());

            let success = match (function, params) {
                (Function::Sphere, FParams::Basic { fopt, xopt }) => {
                    functions::sphere_bbob(ctx, &mut output, x, xopt, *fopt)
                }
                (Function::Ellipsoid, FParams::Basic { fopt, xopt }) => {
                    functions::ellipsoidal_bbob(ctx, &mut output, x, xopt, *fopt)
                }
                (Function::Rastrigin, FParams::Basic { fopt, xopt }) => {
                    functions::rastrigin_bbob(ctx, &mut output, x, xopt, *fopt)
                }
                (Function::BuecheRastrigin, FParams::Basic { fopt, xopt }) => {
                    functions::bueche_rastrigin_bbob(ctx, &mut output, x, xopt, *fopt)
                }
                (Function::LinearSlope, FParams::Basic { fopt, xopt }) => {
                    functions::linear_slope_bbob(ctx, &mut output, x, xopt, *fopt)
                }
                (Function::AttractiveSector, FParams::DoubleRotated { fopt, xopt, R, Q }) => {
                    functions::attractive_sector_bbob(ctx, &mut output, x, xopt, *fopt, R, Q)
                }
                (Function::StepEllipsoid, FParams::DoubleRotated { fopt, xopt, R, Q }) => {
                    functions::step_ellipsoidal_bbob(ctx, &mut output, x, xopt, *fopt, R, Q)
                }
                (Function::Rosenbrock, FParams::Basic { fopt, xopt }) => {
                    functions::rosenbrock_bbob(ctx, &mut output, x, xopt, *fopt)
                }
                (Function::RosenbrockRotated, FParams::FixedRotated { fopt, R }) => {
                    functions::rosenbrock_rotated_bbob(ctx, &mut output, x, *fopt, R)
                }
                (Function::EllipsoidRotated, FParams::Rotated { fopt, xopt, R }) => {
                    functions::ellipsoidal_rotated_bbob(ctx, &mut output, x, xopt, *fopt, R)
                }
                (Function::Discus, FParams::Rotated { fopt, xopt, R }) => {
                    functions::discus_bbob(ctx, &mut output, x, xopt, *fopt, R)
                }
                (Function::BentCigar, FParams::Rotated { fopt, xopt, R }) => {
                    functions::bent_cigar_bbob(ctx, &mut output, x, xopt, *fopt, R)
                }
                (Function::SharpRidge, FParams::DoubleRotated { fopt, xopt, R, Q }) => {
                    functions::sharp_ridge_bbob(ctx, &mut output, x, xopt, *fopt, R, Q)
                }
                (Function::DifferentPowers, FParams::Rotated { fopt, xopt, R }) => {
                    functions::different_powers_bbob(ctx, &mut output, x, xopt, *fopt, R)
                }
                (Function::RastriginRotated, FParams::DoubleRotated { fopt, xopt, R, Q }) => {
                    functions::rastrigin_rotated_bbob(ctx, &mut output, x, xopt, *fopt, R, Q)
                }
                (Function::Weierstrass, FParams::DoubleRotated { fopt, xopt, R, Q }) => {
                    functions::weierstrass_bbob(ctx, &mut output, x, xopt, *fopt, R, Q)
                }
                (Function::Schaffers1, FParams::DoubleRotated { fopt, xopt, R, Q }) => {
                    functions::schaffers_f7_bbob(ctx, &mut output, x, xopt, *fopt, R, Q)
                }
                (Function::Schaffers2, FParams::DoubleRotated { fopt, xopt, R, Q }) => {
                    functions::schaffers_f7_ill_bbob(ctx, &mut output, x, xopt, *fopt, R, Q)
                }
                (Function::GriewankRosenbrock, FParams::FixedRotated { fopt, R }) => {
                    functions::griewank_rosenbrock_bbob(ctx, &mut output, x, *fopt, R)
                }
                (Function::Schwefel, FParams::Basic { fopt, xopt }) => {
                    functions::schwefel_bbob(ctx, &mut output, x, xopt, *fopt)
                }
                (
                    Function::Gallagher1 | Function::Gallagher2,
                    FParams::Gallagher { y, a, fopt, R },
                ) => functions::gallagher(ctx, &mut output, x, y, a, *fopt, R),
                (Function::Katsuura, FParams::DoubleRotated { fopt, xopt, R, Q }) => {
                    functions::katsuura(ctx, &mut output, x, xopt, *fopt, R, Q)
                }
                (Function::LunacekBiRastrigin, FParams::DoubleRotated { fopt, xopt, R, Q }) => {
                    functions::lunacek(ctx, &mut output, x, xopt, *fopt, R, Q)
                }
                _ => panic!("illegal (Function, Params) combination"),
            };

            success.then(|| output)
        }
    };
}

#[macro_export]
macro_rules! declare_problem {
    ($backend:ident) => {
        pub struct Problem<'c> {
            context: &'c $backend::Context,
            function: $crate::Function,
            params: FParams<'c>,
        }

        impl<'c> Problem<'c> {
            pub fn new(
                context: &'c $backend::Context,
                function: $crate::Function,
                params: FParams<'c>,
            ) -> Self {
                Problem {
                    context,
                    function,
                    params,
                }
            }

            pub fn evaluate(&self, x: $crate::InputMatrix) -> Option<Vec<f64>> {
                eval(self.context, self.function, &self.params, x)
            }
        }
    };
}
