use crate::{
    storage::{F64_1D, F64_2D},
    sys, Context,
};

pub type BbobFn = unsafe extern "C" fn(
    ctx: *mut sys::futhark_context,
    y: *mut f64,
    x: *const sys::futhark_f64_1d,
    xopt: *const sys::futhark_f64_1d,
    fopt: f64,
) -> ::std::os::raw::c_int;

pub fn bbob(ctx: &Context, function: BbobFn, x: &F64_1D, xopt: &F64_1D, fopt: f64) -> Option<f64> {
    let mut out = 0f64;
    let out_ptr = &mut out as *mut f64;

    let status = unsafe { (function)(ctx.inner, out_ptr, x.inner, xopt.inner, fopt) == 0 };
    let sync_status = ctx.sync();

    (status && sync_status).then(|| out)
}

pub type RotatedBbobFn = unsafe extern "C" fn(
    ctx: *mut sys::futhark_context,
    y: *mut f64,
    x: *const sys::futhark_f64_1d,
    xopt: *const sys::futhark_f64_1d,
    fopt: f64,
    R: *const sys::futhark_f64_2d,
) -> ::std::os::raw::c_int;

pub fn rotated_bbob(
    ctx: &Context,
    function: RotatedBbobFn,
    x: &F64_1D,
    xopt: &F64_1D,
    fopt: f64,
    R: &F64_2D,
) -> Option<f64> {
    let mut out = 0f64;
    let out_ptr = &mut out as *mut f64;

    let status = unsafe { (function)(ctx.inner, out_ptr, x.inner, xopt.inner, fopt, R.inner) == 0 };
    let sync_status = ctx.sync();

    (status && sync_status).then(|| out)
}

pub type DoubleRotatedBbobFn = unsafe extern "C" fn(
    ctx: *mut sys::futhark_context,
    y: *mut f64,
    x: *const sys::futhark_f64_1d,
    xopt: *const sys::futhark_f64_1d,
    fopt: f64,
    R: *const sys::futhark_f64_2d,
    Q: *const sys::futhark_f64_2d,
) -> ::std::os::raw::c_int;

pub fn double_rotated_bbob(
    ctx: &Context,
    function: DoubleRotatedBbobFn,
    x: &F64_1D,
    xopt: &F64_1D,
    fopt: f64,
    R: &F64_2D,
    Q: &F64_2D,
) -> Option<f64> {
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
