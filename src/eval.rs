use coco_futhark::backends::Backend;

pub enum FutharkParams<'c, B: Backend> {
    Basic {
        fopt: f64,
        xopt: coco_futhark::Array_F64_1D<'c, B>,
    },
    Rotated {
        fopt: f64,
        xopt: coco_futhark::Array_F64_1D<'c, B>,
        R: coco_futhark::Array_F64_2D<'c, B>,
    },
    FixedRotated {
        fopt: f64,
        R: coco_futhark::Array_F64_2D<'c, B>,
    },
    DoubleRotated {
        fopt: f64,
        xopt: coco_futhark::Array_F64_1D<'c, B>,
        R: coco_futhark::Array_F64_2D<'c, B>,
        Q: coco_futhark::Array_F64_2D<'c, B>,
    },
    Gallagher {
        fopt: f64,
        y: coco_futhark::Array_F64_2D<'c, B>,
        w: coco_futhark::Array_F64_1D<'c, B>,
        c: coco_futhark::Array_F64_2D<'c, B>,
        R: coco_futhark::Array_F64_2D<'c, B>,
    },
}
unsafe impl<'c, B: Backend> Send for FutharkParams<'c, B> {}

impl<'c, B: Backend> FutharkParams<'c, B> {
    pub fn fopt(&self) -> f64 {
        *match self {
            FutharkParams::Basic { fopt, .. } => fopt,
            FutharkParams::Rotated { fopt, .. } => fopt,
            FutharkParams::FixedRotated { fopt, .. } => fopt,
            FutharkParams::DoubleRotated { fopt, .. } => fopt,
            FutharkParams::Gallagher { fopt, .. } => fopt,
        }
    }

    pub fn from<'a>(ctx: &'c coco_futhark::Context<B>, params: &crate::Params) -> Self {
        use crate::Params;
        use coco_futhark::{Array_F64_1D, Array_F64_2D};

        match params {
            &Params::Basic { fopt, ref xopt } => {
                let xopt = Array_F64_1D::new(ctx, &xopt, xopt.len());
                FutharkParams::Basic { fopt, xopt }
            }
            &Params::Rotated {
                fopt,
                ref xopt,
                ref R,
            } => {
                let xopt = Array_F64_1D::new(ctx, &xopt, xopt.len());
                let R = Array_F64_2D::new(ctx, &R.data, R.dimension, R.dimension);
                FutharkParams::Rotated { fopt, xopt, R }
            }
            &Params::FixedRotated { fopt, ref R } => {
                let R = Array_F64_2D::new(ctx, &R.data, R.dimension, R.dimension);
                FutharkParams::FixedRotated { fopt, R }
            }
            &Params::DoubleRotated {
                fopt,
                ref xopt,
                ref R,
                ref Q,
            } => {
                let xopt = Array_F64_1D::new(ctx, &xopt, xopt.len());
                let R = Array_F64_2D::new(ctx, &R.data, R.dimension, R.dimension);
                let Q = Array_F64_2D::new(ctx, &Q.data, Q.dimension, Q.dimension);
                FutharkParams::DoubleRotated { fopt, xopt, R, Q }
            }
            &Params::Gallagher {
                fopt,
                peaks,
                ref y,
                ref w,
                ref c,
                ref R,
            } => {
                assert!(
                    peaks == 101 || peaks == 21,
                    "number of peaks must be 101 or 21"
                );
                assert_eq!(
                    peaks * R.dimension,
                    y.len(),
                    "y must have length of dim * peaks"
                );
                let y = Array_F64_2D::new(ctx, &y, R.dimension, peaks);
                let w = Array_F64_1D::new(ctx, &w, w.len());
                let c = Array_F64_2D::new(ctx, &c, peaks, R.dimension);
                let R = Array_F64_2D::new(ctx, &R.data, R.dimension, R.dimension);
                FutharkParams::Gallagher { fopt, y, w, c, R }
            }
        }
    }
}

fn eval<B: Backend>(
    ctx: &coco_futhark::Context<B>,
    function: crate::Function,
    params: &FutharkParams<B>,
    x: crate::InputMatrix,
) -> Option<Vec<f64>> {
    use crate::Function;

    let mut output = Vec::with_capacity(x.inputs());
    let x = &coco_futhark::Array_F64_2D::new(ctx, x.data(), x.inputs(), x.dimension());

    match (function, params) {
        (Function::Sphere, FutharkParams::Basic { fopt, xopt }) => {
            let result = ctx.entry_sphere(x, xopt, *fopt).ok()?;
            result.values(&mut output);
        }
        (Function::Ellipsoid, FutharkParams::Basic { fopt, xopt }) => {
            let result = ctx.entry_ellipsoidal(x, xopt, *fopt).ok()?;
            result.values(&mut output);
        }
        (Function::Rastrigin, FutharkParams::Basic { fopt, xopt }) => {
            let result = ctx.entry_rastrigin(x, xopt, *fopt).ok()?;
            result.values(&mut output);
        }
        (Function::BuecheRastrigin, FutharkParams::Basic { fopt, xopt }) => {
            let result = ctx.entry_bueche_rastrigin(x, xopt, *fopt).ok()?;
            result.values(&mut output);
        }
        (Function::LinearSlope, FutharkParams::Basic { fopt, xopt }) => {
            let result = ctx.entry_linear_slope(x, xopt, *fopt).ok()?;
            result.values(&mut output);
        }
        (Function::AttractiveSector, FutharkParams::Rotated { fopt, xopt, R }) => {
            let result = ctx.entry_attractive_sector(x, xopt, *fopt, R).ok()?;
            result.values(&mut output);
        }
        (Function::StepEllipsoid, FutharkParams::DoubleRotated { fopt, xopt, R, Q }) => {
            let result = ctx.entry_step_ellipsoidal(x, xopt, *fopt, R, Q).ok()?;
            result.values(&mut output);
        }
        (Function::Rosenbrock, FutharkParams::Basic { fopt, xopt }) => {
            let result = ctx.entry_rosenbrock(x, xopt, *fopt).ok()?;
            result.values(&mut output);
        }
        (Function::RosenbrockRotated, FutharkParams::FixedRotated { fopt, R }) => {
            let result = ctx.entry_rosenbrock_rotated(x, *fopt, R).ok()?;
            result.values(&mut output);
        }
        (Function::EllipsoidRotated, FutharkParams::Rotated { fopt, xopt, R }) => {
            let result = ctx.entry_ellipsoidal_rotated(x, xopt, *fopt, R).ok()?;
            result.values(&mut output);
        }
        (Function::Discus, FutharkParams::Rotated { fopt, xopt, R }) => {
            let result = ctx.entry_discus(x, xopt, *fopt, R).ok()?;
            result.values(&mut output);
        }
        (Function::BentCigar, FutharkParams::Rotated { fopt, xopt, R }) => {
            let result = ctx.entry_bent_cigar(x, xopt, *fopt, R).ok()?;
            result.values(&mut output);
        }
        (Function::SharpRidge, FutharkParams::Rotated { fopt, xopt, R }) => {
            let result = ctx.entry_sharp_ridge(x, xopt, *fopt, R).ok()?;
            result.values(&mut output);
        }
        (Function::DifferentPowers, FutharkParams::Rotated { fopt, xopt, R }) => {
            let result = ctx.entry_different_powers(x, xopt, *fopt, R).ok()?;
            result.values(&mut output);
        }
        (Function::RastriginRotated, FutharkParams::DoubleRotated { fopt, xopt, R, Q }) => {
            let result = ctx.entry_rastrigin_rotated(x, xopt, *fopt, R, Q).ok()?;
            result.values(&mut output);
        }
        (Function::Weierstrass, FutharkParams::DoubleRotated { fopt, xopt, R, Q }) => {
            let result = ctx.entry_weierstrass(x, xopt, *fopt, R, Q).ok()?;
            result.values(&mut output);
        }
        (Function::Schaffers1, FutharkParams::DoubleRotated { fopt, xopt, R, Q }) => {
            let result = ctx.entry_schaffers_f7(x, xopt, *fopt, R, Q).ok()?;
            result.values(&mut output);
        }
        (Function::Schaffers2, FutharkParams::DoubleRotated { fopt, xopt, R, Q }) => {
            let result = ctx.entry_schaffers_f7_ill(x, xopt, *fopt, R, Q).ok()?;
            result.values(&mut output);
        }
        (Function::GriewankRosenbrock, FutharkParams::FixedRotated { fopt, R }) => {
            let result = ctx.entry_griewank_rosenbrock(x, *fopt, R).ok()?;
            result.values(&mut output);
        }
        (Function::Schwefel, FutharkParams::Basic { fopt, xopt }) => {
            let result = ctx.entry_schwefel(x, xopt, *fopt).ok()?;
            result.values(&mut output);
        }
        (
            Function::Gallagher1 | Function::Gallagher2,
            FutharkParams::Gallagher { y, w, c, fopt, R },
        ) => {
            let result = ctx.entry_gallagher(x, y, w, c, *fopt, R).ok()?;
            result.values(&mut output);
        }
        (Function::Katsuura, FutharkParams::DoubleRotated { fopt, xopt, R, Q }) => {
            let result = ctx.entry_katsuura(x, xopt, *fopt, R, Q).ok()?;
            result.values(&mut output);
        }
        (Function::LunacekBiRastrigin, FutharkParams::Rotated { fopt, xopt, R }) => {
            let result = ctx.entry_lunacek(x, xopt, *fopt, R).ok()?;
            result.values(&mut output);
        }
        _ => panic!("illegal (Function, Params) combination"),
    };

    Some(output)
}
pub struct Problem<'c, B: Backend> {
    context: &'c coco_futhark::Context<B>,
    function: crate::Function,
    params: FutharkParams<'c, B>,
}
impl<'c, B: Backend> Problem<'c, B> {
    pub fn new(
        context: &'c coco_futhark::Context<B>,
        function: crate::Function,
        params: FutharkParams<'c, B>,
    ) -> Self {
        Problem {
            context,
            function,
            params,
        }
    }
    pub fn target_hit(&self, value: f64) -> bool {
        value <= self.params.fopt() + 1e-8
    }
    pub fn evaluate(&self, x: crate::InputMatrix) -> Option<Vec<f64>> {
        eval(self.context, self.function, &self.params, x)
    }
}
