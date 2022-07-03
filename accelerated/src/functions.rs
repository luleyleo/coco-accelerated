use crate::{
    storage::{F64_1D, F64_2D},
    sys, Context,
};

type BbobFn = unsafe extern "C" fn(
    ctx: *mut sys::futhark_context,
    y: *mut f64,
    x: *const sys::futhark_f64_1d,
    xopt: *const sys::futhark_f64_1d,
    fopt: f64,
) -> ::std::os::raw::c_int;

fn run_bbob(ctx: &Context, function: BbobFn, x: &F64_1D, xopt: &F64_1D, fopt: f64) -> Option<f64> {
    let mut out = 0f64;
    let out_ptr = &mut out as *mut f64;

    let status = unsafe { (function)(ctx.inner, out_ptr, x.inner, xopt.inner, fopt) == 0 };
    let sync_status = ctx.sync();

    (status && sync_status).then(|| out)
}

pub fn sphere_bbob(ctx: &Context, x: &F64_1D, xopt: &F64_1D, fopt: f64) -> Option<f64> {
    run_bbob(ctx, sys::futhark_entry_sphere, x, xopt, fopt)
}

pub fn ellipsoidal_bbob(ctx: &Context, x: &F64_1D, xopt: &F64_1D, fopt: f64) -> Option<f64> {
    run_bbob(ctx, sys::futhark_entry_ellipsoidal, x, xopt, fopt)
}

pub fn rastrigin_bbob(ctx: &Context, x: &F64_1D, xopt: &F64_1D, fopt: f64) -> Option<f64> {
    run_bbob(ctx, sys::futhark_entry_rastrigin, x, xopt, fopt)
}

pub fn bueche_rastrigin_bbob(ctx: &Context, x: &F64_1D, xopt: &F64_1D, fopt: f64) -> Option<f64> {
    run_bbob(ctx, sys::futhark_entry_bueche_rastrigin, x, xopt, fopt)
}

pub fn linear_slope_bbob(ctx: &Context, x: &F64_1D, xopt: &F64_1D, fopt: f64) -> Option<f64> {
    run_bbob(ctx, sys::futhark_entry_linear_slope, x, xopt, fopt)
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
    {
        let function = sys::futhark_entry_step_ellipsoidal;

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

pub fn rosenbrock_bbob(ctx: &Context, x: &F64_1D, xopt: &F64_1D, fopt: f64) -> Option<f64> {
    run_bbob(ctx, sys::futhark_entry_rosenbrock, x, xopt, fopt)
}

pub fn rosenbrock_rotated_bbob(ctx: &Context, x: &F64_1D, R: &F64_2D, fopt: f64) -> Option<f64> {
    let function = sys::futhark_entry_rosenbrock_rotated;

    let mut out = 0f64;
    let out_ptr = &mut out as *mut f64;

    let status = unsafe { (function)(ctx.inner, out_ptr, x.inner, R.inner, fopt) == 0 };
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
    {
        let function = sys::futhark_entry_ellipsoidal_rotated;

        let mut out = 0f64;
        let out_ptr = &mut out as *mut f64;

        let status =
            unsafe { (function)(ctx.inner, out_ptr, x.inner, xopt.inner, fopt, R.inner) == 0 };
        let sync_status = ctx.sync();

        (status && sync_status).then(|| out)
    }
}
