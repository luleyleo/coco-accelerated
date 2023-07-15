use crate::{
    backend::{types, Backend},
    run,
    storage::{F64_1D, F64_2D},
    Context,
};

pub fn sphere_bbob<B: Backend>(
    ctx: &Context<B>,
    output: &mut Vec<f64>,
    x: &F64_2D<B>,
    xopt: &F64_1D<B>,
    fopt: f64,
) -> bool {
    run::bbob(ctx, B::futhark_entry_sphere, output, x, xopt, fopt)
}

pub fn ellipsoidal_bbob<B: Backend>(
    ctx: &Context<B>,
    output: &mut Vec<f64>,
    x: &F64_2D<B>,
    xopt: &F64_1D<B>,
    fopt: f64,
) -> bool {
    run::bbob(ctx, B::futhark_entry_ellipsoidal, output, x, xopt, fopt)
}

pub fn rastrigin_bbob<B: Backend>(
    ctx: &Context<B>,
    output: &mut Vec<f64>,
    x: &F64_2D<B>,
    xopt: &F64_1D<B>,
    fopt: f64,
) -> bool {
    run::bbob(ctx, B::futhark_entry_rastrigin, output, x, xopt, fopt)
}

pub fn bueche_rastrigin_bbob<B: Backend>(
    ctx: &Context<B>,
    output: &mut Vec<f64>,
    x: &F64_2D<B>,
    xopt: &F64_1D<B>,
    fopt: f64,
) -> bool {
    run::bbob(
        ctx,
        B::futhark_entry_bueche_rastrigin,
        output,
        x,
        xopt,
        fopt,
    )
}

pub fn linear_slope_bbob<B: Backend>(
    ctx: &Context<B>,
    output: &mut Vec<f64>,
    x: &F64_2D<B>,
    xopt: &F64_1D<B>,
    fopt: f64,
) -> bool {
    run::bbob(ctx, B::futhark_entry_linear_slope, output, x, xopt, fopt)
}

pub fn attractive_sector_bbob<B: Backend>(
    ctx: &Context<B>,
    output: &mut Vec<f64>,
    x: &F64_2D<B>,
    xopt: &F64_1D<B>,
    fopt: f64,
    R: &F64_2D<B>,
) -> bool {
    run::rotated_bbob(
        ctx,
        B::futhark_entry_attractive_sector,
        output,
        x,
        xopt,
        fopt,
        R,
    )
}

pub fn step_ellipsoidal_bbob<B: Backend>(
    ctx: &Context<B>,
    output: &mut Vec<f64>,
    x: &F64_2D<B>,
    xopt: &F64_1D<B>,
    fopt: f64,
    R: &F64_2D<B>,
    Q: &F64_2D<B>,
) -> bool {
    run::double_rotated_bbob(
        ctx,
        B::futhark_entry_step_ellipsoidal,
        output,
        x,
        xopt,
        fopt,
        R,
        Q,
    )
}

pub fn rosenbrock_bbob<B: Backend>(
    ctx: &Context<B>,
    output: &mut Vec<f64>,
    x: &F64_2D<B>,
    xopt: &F64_1D<B>,
    fopt: f64,
) -> bool {
    run::bbob(ctx, B::futhark_entry_rosenbrock, output, x, xopt, fopt)
}

pub fn rosenbrock_rotated_bbob<B: Backend>(
    ctx: &Context<B>,
    output: &mut Vec<f64>,
    x: &F64_2D<B>,
    fopt: f64,
    R: &F64_2D<B>,
) -> bool {
    let function = B::futhark_entry_rosenbrock_rotated;

    let mut out: *mut types::futhark_f64_1d = std::ptr::null_mut();

    let status = unsafe { (function)(ctx.inner, &mut out, x.inner, fopt, R.inner) == 0 };
    let sync_status = ctx.sync();

    if status && sync_status {
        unsafe {
            F64_1D::from_raw(ctx, out).values(output);
        }

        true
    } else {
        false
    }
}

pub fn ellipsoidal_rotated_bbob<B: Backend>(
    ctx: &Context<B>,
    output: &mut Vec<f64>,
    x: &F64_2D<B>,
    xopt: &F64_1D<B>,
    fopt: f64,
    R: &F64_2D<B>,
) -> bool {
    run::rotated_bbob(
        ctx,
        B::futhark_entry_ellipsoidal_rotated,
        output,
        x,
        xopt,
        fopt,
        R,
    )
}

pub fn discus_bbob<B: Backend>(
    ctx: &Context<B>,
    output: &mut Vec<f64>,
    x: &F64_2D<B>,
    xopt: &F64_1D<B>,
    fopt: f64,
    R: &F64_2D<B>,
) -> bool {
    run::rotated_bbob(ctx, B::futhark_entry_discus, output, x, xopt, fopt, R)
}

pub fn bent_cigar_bbob<B: Backend>(
    ctx: &Context<B>,
    output: &mut Vec<f64>,
    x: &F64_2D<B>,
    xopt: &F64_1D<B>,
    fopt: f64,
    R: &F64_2D<B>,
) -> bool {
    run::rotated_bbob(ctx, B::futhark_entry_bent_cigar, output, x, xopt, fopt, R)
}

pub fn sharp_ridge_bbob<B: Backend>(
    ctx: &Context<B>,
    output: &mut Vec<f64>,
    x: &F64_2D<B>,
    xopt: &F64_1D<B>,
    fopt: f64,
    M: &F64_2D<B>,
) -> bool {
    run::rotated_bbob(ctx, B::futhark_entry_sharp_ridge, output, x, xopt, fopt, M)
}

pub fn different_powers_bbob<B: Backend>(
    ctx: &Context<B>,
    output: &mut Vec<f64>,
    x: &F64_2D<B>,
    xopt: &F64_1D<B>,
    fopt: f64,
    R: &F64_2D<B>,
) -> bool {
    run::rotated_bbob(
        ctx,
        B::futhark_entry_different_powers,
        output,
        x,
        xopt,
        fopt,
        R,
    )
}

pub fn rastrigin_rotated_bbob<B: Backend>(
    ctx: &Context<B>,
    output: &mut Vec<f64>,
    x: &F64_2D<B>,
    xopt: &F64_1D<B>,
    fopt: f64,
    R: &F64_2D<B>,
    Q: &F64_2D<B>,
) -> bool {
    run::double_rotated_bbob(
        ctx,
        B::futhark_entry_rastrigin_rotated,
        output,
        x,
        xopt,
        fopt,
        R,
        Q,
    )
}

pub fn weierstrass_bbob<B: Backend>(
    ctx: &Context<B>,
    output: &mut Vec<f64>,
    x: &F64_2D<B>,
    xopt: &F64_1D<B>,
    fopt: f64,
    R: &F64_2D<B>,
    Q: &F64_2D<B>,
) -> bool {
    run::double_rotated_bbob(
        ctx,
        B::futhark_entry_weierstrass,
        output,
        x,
        xopt,
        fopt,
        R,
        Q,
    )
}

pub fn schaffers_f7_bbob<B: Backend>(
    ctx: &Context<B>,
    output: &mut Vec<f64>,
    x: &F64_2D<B>,
    xopt: &F64_1D<B>,
    fopt: f64,
    R: &F64_2D<B>,
    Q: &F64_2D<B>,
) -> bool {
    run::double_rotated_bbob(
        ctx,
        B::futhark_entry_schaffers_f7,
        output,
        x,
        xopt,
        fopt,
        R,
        Q,
    )
}

pub fn schaffers_f7_ill_bbob<B: Backend>(
    ctx: &Context<B>,
    output: &mut Vec<f64>,
    x: &F64_2D<B>,
    xopt: &F64_1D<B>,
    fopt: f64,
    R: &F64_2D<B>,
    Q: &F64_2D<B>,
) -> bool {
    run::double_rotated_bbob(
        ctx,
        B::futhark_entry_schaffers_f7_ill,
        output,
        x,
        xopt,
        fopt,
        R,
        Q,
    )
}

pub fn griewank_rosenbrock_bbob<B: Backend>(
    ctx: &Context<B>,
    output: &mut Vec<f64>,
    x: &F64_2D<B>,
    fopt: f64,
    R: &F64_2D<B>,
) -> bool {
    let function = B::futhark_entry_griewank_rosenbrock;

    let mut out: *mut types::futhark_f64_1d = std::ptr::null_mut();

    let status = unsafe { (function)(ctx.inner, &mut out, x.inner, fopt, R.inner) == 0 };
    let sync_status = ctx.sync();

    if status && sync_status {
        unsafe {
            F64_1D::from_raw(ctx, out).values(output);
        }

        true
    } else {
        false
    }
}

pub fn schwefel_bbob<B: Backend>(
    ctx: &Context<B>,
    output: &mut Vec<f64>,
    x: &F64_2D<B>,
    xopt: &F64_1D<B>,
    fopt: f64,
) -> bool {
    run::bbob(ctx, B::futhark_entry_schwefel, output, x, xopt, fopt)
}

pub fn gallagher<B: Backend>(
    ctx: &Context<B>,
    output: &mut Vec<f64>,
    x: &F64_2D<B>,
    y: &F64_2D<B>,
    w: &F64_1D<B>,
    c: &F64_2D<B>,
    fopt: f64,
    R: &F64_2D<B>,
) -> bool {
    let function = B::futhark_entry_gallagher;

    let mut out: *mut types::futhark_f64_1d = std::ptr::null_mut();

    let status = unsafe {
        (function)(
            ctx.inner, &mut out, x.inner, y.inner, w.inner, c.inner, fopt, R.inner,
        ) == 0
    };
    let sync_status = ctx.sync();

    if status && sync_status && !out.is_null() {
        unsafe {
            F64_1D::from_raw(ctx, out).values(output);
        }

        true
    } else {
        false
    }
}

pub fn katsuura<B: Backend>(
    ctx: &Context<B>,
    output: &mut Vec<f64>,
    x: &F64_2D<B>,
    xopt: &F64_1D<B>,
    fopt: f64,
    R: &F64_2D<B>,
    Q: &F64_2D<B>,
) -> bool {
    run::double_rotated_bbob(ctx, B::futhark_entry_katsuura, output, x, xopt, fopt, R, Q)
}

pub fn lunacek<B: Backend>(
    ctx: &Context<B>,
    output: &mut Vec<f64>,
    x: &F64_2D<B>,
    xopt: &F64_1D<B>,
    fopt: f64,
    R: &F64_2D<B>,
) -> bool {
    run::rotated_bbob(ctx, B::futhark_entry_lunacek, output, x, xopt, fopt, R)
}
