use coco_futhark_opencl::{functions, storage};

use crate::{eval::eval_futhark, Context, Function, InputMatrix, Problem};

pub fn futhark_opencl(problem: &Problem, x: InputMatrix) -> Vec<f64> {
    eval_futhark(eval_futhark_opencl, problem, x)
}

fn eval_futhark_opencl(
    ctx: &Context,
    x: InputMatrix,
    xopt: Vec<f64>,
    R: coco_legacy::Matrix,
    Q: coco_legacy::Matrix,
    function: Function,
    fopt: f64,
) -> Option<Vec<f64>> {
    let ctx = &ctx.futhark_opencl;

    let mut output = Vec::with_capacity(x.inputs());
    let x = &x.allocate_futhark_opencl_array(ctx);
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
        Function::SharpRidge => functions::sharp_ridge_bbob(ctx, &mut output, x, xopt, fopt, R, Q),
        Function::DifferentPowers => {
            functions::different_powers_bbob(ctx, &mut output, x, xopt, fopt, R)
        }
        Function::RastriginRotated => {
            functions::rastrigin_rotated_bbob(ctx, &mut output, x, xopt, fopt, R, Q)
        }
        Function::Weierstrass => functions::weierstrass_bbob(ctx, &mut output, x, xopt, fopt, R, Q),
        Function::Schaffers1 => functions::schaffers_f7_bbob(ctx, &mut output, x, xopt, fopt, R, Q),
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
}
