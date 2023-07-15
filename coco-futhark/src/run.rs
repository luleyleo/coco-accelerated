use crate::{
    storage::{F64_1D, F64_2D},
    sys, Context,
};

pub type BbobFn = unsafe extern "C" fn(
    ctx: *mut sys::futhark_context,
    y: *mut *mut sys::futhark_f64_1d,
    x: *const sys::futhark_f64_2d,
    xopt: *const sys::futhark_f64_1d,
    fopt: f64,
) -> ::std::os::raw::c_int;

pub fn bbob(
    ctx: &Context,
    function: BbobFn,
    output: &mut Vec<f64>,
    x: &F64_2D,
    xopt: &F64_1D,
    fopt: f64,
) -> bool {
    let mut out: *mut sys::futhark_f64_1d = std::ptr::null_mut();

    let status = unsafe { (function)(ctx.inner, &mut out, x.inner, xopt.inner, fopt) == 0 };
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

pub type RotatedBbobFn = unsafe extern "C" fn(
    ctx: *mut sys::futhark_context,
    y: *mut *mut sys::futhark_f64_1d,
    x: *const sys::futhark_f64_2d,
    xopt: *const sys::futhark_f64_1d,
    fopt: f64,
    R: *const sys::futhark_f64_2d,
) -> ::std::os::raw::c_int;

pub fn rotated_bbob(
    ctx: &Context,
    function: RotatedBbobFn,
    output: &mut Vec<f64>,
    x: &F64_2D,
    xopt: &F64_1D,
    fopt: f64,
    R: &F64_2D,
) -> bool {
    let mut out: *mut sys::futhark_f64_1d = std::ptr::null_mut();

    let status =
        unsafe { (function)(ctx.inner, &mut out, x.inner, xopt.inner, fopt, R.inner) == 0 };
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

pub type DoubleRotatedBbobFn = unsafe extern "C" fn(
    ctx: *mut sys::futhark_context,
    y: *mut *mut sys::futhark_f64_1d,
    x: *const sys::futhark_f64_2d,
    xopt: *const sys::futhark_f64_1d,
    fopt: f64,
    R: *const sys::futhark_f64_2d,
    Q: *const sys::futhark_f64_2d,
) -> ::std::os::raw::c_int;

pub fn double_rotated_bbob(
    ctx: &Context,
    function: DoubleRotatedBbobFn,
    output: &mut Vec<f64>,
    x: &F64_2D,
    xopt: &F64_1D,
    fopt: f64,
    R: &F64_2D,
    Q: &F64_2D,
) -> bool {
    let mut out: *mut sys::futhark_f64_1d = std::ptr::null_mut();

    let status = unsafe {
        (function)(
            ctx.inner, &mut out, x.inner, xopt.inner, fopt, R.inner, Q.inner,
        ) == 0
    };
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
