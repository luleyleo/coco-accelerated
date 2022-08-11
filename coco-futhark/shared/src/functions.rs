use crate::{
    run,
    storage::{F64_1D, F64_2D},
    sys, Context,
};

pub fn sphere_bbob(
    ctx: &Context,
    output: &mut Vec<f64>,
    x: &F64_2D,
    xopt: &F64_1D,
    fopt: f64,
) -> bool {
    run::bbob(ctx, sys::futhark_entry_sphere, output, x, xopt, fopt)
}

pub fn ellipsoidal_bbob(
    ctx: &Context,
    output: &mut Vec<f64>,
    x: &F64_2D,
    xopt: &F64_1D,
    fopt: f64,
) -> bool {
    run::bbob(ctx, sys::futhark_entry_ellipsoidal, output, x, xopt, fopt)
}

pub fn rastrigin_bbob(
    ctx: &Context,
    output: &mut Vec<f64>,
    x: &F64_2D,
    xopt: &F64_1D,
    fopt: f64,
) -> bool {
    run::bbob(ctx, sys::futhark_entry_rastrigin, output, x, xopt, fopt)
}

pub fn bueche_rastrigin_bbob(
    ctx: &Context,
    output: &mut Vec<f64>,
    x: &F64_2D,
    xopt: &F64_1D,
    fopt: f64,
) -> bool {
    run::bbob(
        ctx,
        sys::futhark_entry_bueche_rastrigin,
        output,
        x,
        xopt,
        fopt,
    )
}

pub fn linear_slope_bbob(
    ctx: &Context,
    output: &mut Vec<f64>,
    x: &F64_2D,
    xopt: &F64_1D,
    fopt: f64,
) -> bool {
    run::bbob(ctx, sys::futhark_entry_linear_slope, output, x, xopt, fopt)
}

pub fn attractive_sector_bbob(
    ctx: &Context,
    output: &mut Vec<f64>,
    x: &F64_2D,
    xopt: &F64_1D,
    fopt: f64,
    R: &F64_2D,
    Q: &F64_2D,
) -> bool {
    run::double_rotated_bbob(
        ctx,
        sys::futhark_entry_attractive_sector,
        output,
        x,
        xopt,
        fopt,
        R,
        Q,
    )
}

pub fn step_ellipsoidal_bbob(
    ctx: &Context,
    output: &mut Vec<f64>,
    x: &F64_2D,
    xopt: &F64_1D,
    fopt: f64,
    R: &F64_2D,
    Q: &F64_2D,
) -> bool {
    run::double_rotated_bbob(
        ctx,
        sys::futhark_entry_step_ellipsoidal,
        output,
        x,
        xopt,
        fopt,
        R,
        Q,
    )
}

pub fn rosenbrock_bbob(
    ctx: &Context,
    output: &mut Vec<f64>,
    x: &F64_2D,
    xopt: &F64_1D,
    fopt: f64,
) -> bool {
    run::bbob(ctx, sys::futhark_entry_rosenbrock, output, x, xopt, fopt)
}

pub fn rosenbrock_rotated_bbob(
    ctx: &Context,
    output: &mut Vec<f64>,
    x: &F64_2D,
    fopt: f64,
    R: &F64_2D,
) -> bool {
    let function = sys::futhark_entry_rosenbrock_rotated;

    let mut out: *mut sys::futhark_f64_1d = std::ptr::null_mut();

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

pub fn ellipsoidal_rotated_bbob(
    ctx: &Context,
    output: &mut Vec<f64>,
    x: &F64_2D,
    xopt: &F64_1D,
    fopt: f64,
    R: &F64_2D,
) -> bool {
    run::rotated_bbob(
        ctx,
        sys::futhark_entry_ellipsoidal_rotated,
        output,
        x,
        xopt,
        fopt,
        R,
    )
}

pub fn discus_bbob(
    ctx: &Context,
    output: &mut Vec<f64>,
    x: &F64_2D,
    xopt: &F64_1D,
    fopt: f64,
    R: &F64_2D,
) -> bool {
    run::rotated_bbob(ctx, sys::futhark_entry_discus, output, x, xopt, fopt, R)
}

pub fn bent_cigar_bbob(
    ctx: &Context,
    output: &mut Vec<f64>,
    x: &F64_2D,
    xopt: &F64_1D,
    fopt: f64,
    R: &F64_2D,
) -> bool {
    run::rotated_bbob(ctx, sys::futhark_entry_bent_cigar, output, x, xopt, fopt, R)
}

pub fn sharp_ridge_bbob(
    ctx: &Context,
    output: &mut Vec<f64>,
    x: &F64_2D,
    xopt: &F64_1D,
    fopt: f64,
    R: &F64_2D,
    Q: &F64_2D,
) -> bool {
    run::double_rotated_bbob(
        ctx,
        sys::futhark_entry_sharp_ridge,
        output,
        x,
        xopt,
        fopt,
        R,
        Q,
    )
}

pub fn different_powers_bbob(
    ctx: &Context,
    output: &mut Vec<f64>,
    x: &F64_2D,
    xopt: &F64_1D,
    fopt: f64,
    R: &F64_2D,
) -> bool {
    run::rotated_bbob(
        ctx,
        sys::futhark_entry_different_powers,
        output,
        x,
        xopt,
        fopt,
        R,
    )
}

pub fn rastrigin_rotated_bbob(
    ctx: &Context,
    output: &mut Vec<f64>,
    x: &F64_2D,
    xopt: &F64_1D,
    fopt: f64,
    R: &F64_2D,
    Q: &F64_2D,
) -> bool {
    run::double_rotated_bbob(
        ctx,
        sys::futhark_entry_rastrigin_rotated,
        output,
        x,
        xopt,
        fopt,
        R,
        Q,
    )
}

pub fn weierstrass_bbob(
    ctx: &Context,
    output: &mut Vec<f64>,
    x: &F64_2D,
    xopt: &F64_1D,
    fopt: f64,
    R: &F64_2D,
    Q: &F64_2D,
) -> bool {
    run::double_rotated_bbob(
        ctx,
        sys::futhark_entry_weierstrass,
        output,
        x,
        xopt,
        fopt,
        R,
        Q,
    )
}

pub fn schaffers_f7_bbob(
    ctx: &Context,
    output: &mut Vec<f64>,
    x: &F64_2D,
    xopt: &F64_1D,
    fopt: f64,
    R: &F64_2D,
    Q: &F64_2D,
) -> bool {
    run::double_rotated_bbob(
        ctx,
        sys::futhark_entry_schaffers_f7,
        output,
        x,
        xopt,
        fopt,
        R,
        Q,
    )
}

pub fn schaffers_f7_ill_bbob(
    ctx: &Context,
    output: &mut Vec<f64>,
    x: &F64_2D,
    xopt: &F64_1D,
    fopt: f64,
    R: &F64_2D,
    Q: &F64_2D,
) -> bool {
    run::double_rotated_bbob(
        ctx,
        sys::futhark_entry_schaffers_f7_ill,
        output,
        x,
        xopt,
        fopt,
        R,
        Q,
    )
}

pub fn griewank_rosenbrock_bbob(
    ctx: &Context,
    output: &mut Vec<f64>,
    x: &F64_2D,
    fopt: f64,
    R: &F64_2D,
) -> bool {
    let function = sys::futhark_entry_griewank_rosenbrock;

    let mut out: *mut sys::futhark_f64_1d = std::ptr::null_mut();

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

pub fn schwefel_bbob(
    ctx: &Context,
    output: &mut Vec<f64>,
    x: &F64_2D,
    xopt: &F64_1D,
    fopt: f64,
) -> bool {
    run::bbob(ctx, sys::futhark_entry_schwefel, output, x, xopt, fopt)
}

pub fn gallagher(
    ctx: &Context,
    output: &mut Vec<f64>,
    x: &F64_2D,
    y: &F64_2D,
    a: &F64_1D,
    fopt: f64,
    R: &F64_2D,
) -> bool {
    let function = sys::futhark_entry_gallagher;

    let mut out: *mut sys::futhark_f64_1d = std::ptr::null_mut();

    let status = unsafe {
        (function)(
            ctx.inner, &mut out, x.inner, y.inner, a.inner, fopt, R.inner,
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
