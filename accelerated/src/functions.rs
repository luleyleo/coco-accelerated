use crate::{
    run,
    storage::{F64_1D, F64_2D},
    sys, Context,
};

pub fn sphere_bbob(ctx: &Context, x: &F64_1D, xopt: &F64_1D, fopt: f64) -> Option<f64> {
    run::bbob(ctx, sys::futhark_entry_sphere, x, xopt, fopt)
}

pub fn ellipsoidal_bbob(ctx: &Context, x: &F64_1D, xopt: &F64_1D, fopt: f64) -> Option<f64> {
    run::bbob(ctx, sys::futhark_entry_ellipsoidal, x, xopt, fopt)
}

pub fn rastrigin_bbob(ctx: &Context, x: &F64_1D, xopt: &F64_1D, fopt: f64) -> Option<f64> {
    run::bbob(ctx, sys::futhark_entry_rastrigin, x, xopt, fopt)
}

pub fn bueche_rastrigin_bbob(ctx: &Context, x: &F64_1D, xopt: &F64_1D, fopt: f64) -> Option<f64> {
    run::bbob(ctx, sys::futhark_entry_bueche_rastrigin, x, xopt, fopt)
}

pub fn linear_slope_bbob(ctx: &Context, x: &F64_1D, xopt: &F64_1D, fopt: f64) -> Option<f64> {
    run::bbob(ctx, sys::futhark_entry_linear_slope, x, xopt, fopt)
}

pub fn attractive_sector_bbob(
    ctx: &Context,
    x: &F64_1D,
    xopt: &F64_1D,
    fopt: f64,
    R: &F64_2D,
    Q: &F64_2D,
) -> Option<f64> {
    {
        let function = sys::futhark_entry_attractive_sector;

        let mut out = 0f64;
        let out_ptr = &mut out as *mut f64;

        let status = unsafe {
            (function)(
                ctx.inner, out_ptr, x.inner, xopt.inner, fopt, R.inner, Q.inner,
            ) == 0
        };
        let sync_status = ctx.sync();

        (status && sync_status).then(|| out)
    }
}

pub fn step_ellipsoidal_bbob(
    ctx: &Context,
    x: &F64_1D,
    xopt: &F64_1D,
    fopt: f64,
    R: &F64_2D,
    Q: &F64_2D,
) -> Option<f64> {
    run::double_rotated_bbob(
        ctx,
        sys::futhark_entry_step_ellipsoidal,
        x,
        xopt,
        fopt,
        R,
        Q,
    )
}

pub fn rosenbrock_bbob(ctx: &Context, x: &F64_1D, xopt: &F64_1D, fopt: f64) -> Option<f64> {
    run::bbob(ctx, sys::futhark_entry_rosenbrock, x, xopt, fopt)
}

pub fn rosenbrock_rotated_bbob(ctx: &Context, x: &F64_1D, fopt: f64, R: &F64_2D) -> Option<f64> {
    let function = sys::futhark_entry_rosenbrock_rotated;

    let mut out = 0f64;
    let out_ptr = &mut out as *mut f64;

    let status = unsafe { (function)(ctx.inner, out_ptr, x.inner, fopt, R.inner) == 0 };
    let sync_status = ctx.sync();

    (status && sync_status).then(|| out)
}

pub fn ellipsoidal_rotated_bbob(
    ctx: &Context,
    x: &F64_1D,
    xopt: &F64_1D,
    fopt: f64,
    R: &F64_2D,
) -> Option<f64> {
    run::rotated_bbob(
        ctx,
        sys::futhark_entry_ellipsoidal_rotated,
        x,
        xopt,
        fopt,
        R,
    )
}

pub fn discus_bbob(ctx: &Context, x: &F64_1D, xopt: &F64_1D, fopt: f64, R: &F64_2D) -> Option<f64> {
    run::rotated_bbob(ctx, sys::futhark_entry_discus, x, xopt, fopt, R)
}

pub fn bent_cigar_bbob(
    ctx: &Context,
    x: &F64_1D,
    xopt: &F64_1D,
    fopt: f64,
    R: &F64_2D,
) -> Option<f64> {
    run::rotated_bbob(ctx, sys::futhark_entry_bent_cigar, x, xopt, fopt, R)
}

pub fn sharp_ridge_bbob(
    ctx: &Context,
    x: &F64_1D,
    xopt: &F64_1D,
    fopt: f64,
    R: &F64_2D,
    Q: &F64_2D,
) -> Option<f64> {
    run::double_rotated_bbob(ctx, sys::futhark_entry_sharp_ridge, x, xopt, fopt, R, Q)
}

pub fn different_powers_bbob(
    ctx: &Context,
    x: &F64_1D,
    xopt: &F64_1D,
    fopt: f64,
    R: &F64_2D,
) -> Option<f64> {
    run::rotated_bbob(ctx, sys::futhark_entry_different_powers, x, xopt, fopt, R)
}

pub fn rastrigin_rotated_bbob(
    ctx: &Context,
    x: &F64_1D,
    xopt: &F64_1D,
    fopt: f64,
    R: &F64_2D,
    Q: &F64_2D,
) -> Option<f64> {
    run::double_rotated_bbob(
        ctx,
        sys::futhark_entry_rastrigin_rotated,
        x,
        xopt,
        fopt,
        R,
        Q,
    )
}

pub fn weierstrass_bbob(
    ctx: &Context,
    x: &F64_1D,
    xopt: &F64_1D,
    fopt: f64,
    R: &F64_2D,
    Q: &F64_2D,
) -> Option<f64> {
    run::double_rotated_bbob(ctx, sys::futhark_entry_weierstrass, x, xopt, fopt, R, Q)
}

pub fn schaffers_f7_bbob(
    ctx: &Context,
    x: &F64_1D,
    xopt: &F64_1D,
    fopt: f64,
    R: &F64_2D,
    Q: &F64_2D,
) -> Option<f64> {
    run::double_rotated_bbob(ctx, sys::futhark_entry_schaffers_f7, x, xopt, fopt, R, Q)
}

pub fn schaffers_f7_ill_bbob(
    ctx: &Context,
    x: &F64_1D,
    xopt: &F64_1D,
    fopt: f64,
    R: &F64_2D,
    Q: &F64_2D,
) -> Option<f64> {
    run::double_rotated_bbob(
        ctx,
        sys::futhark_entry_schaffers_f7_ill,
        x,
        xopt,
        fopt,
        R,
        Q,
    )
}

pub fn griewank_rosenbrock_bbob(ctx: &Context, x: &F64_1D, fopt: f64, R: &F64_2D) -> Option<f64> {
    let function = sys::futhark_entry_griewank_rosenbrock;

    let mut out = 0f64;
    let out_ptr = &mut out as *mut f64;

    let status = unsafe { (function)(ctx.inner, out_ptr, x.inner, fopt, R.inner) == 0 };
    let sync_status = ctx.sync();

    (status && sync_status).then(|| out)
}
