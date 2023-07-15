use crate::{
    backend::{types, Backend},
    storage::{F64_1D, F64_2D},
    Context,
};

pub type BbobFn = unsafe fn(
    ctx: *mut types::futhark_context,
    y: *mut *mut types::futhark_f64_1d,
    x: *const types::futhark_f64_2d,
    xopt: *const types::futhark_f64_1d,
    fopt: f64,
) -> ::std::os::raw::c_int;

pub fn bbob<B: Backend>(
    ctx: &Context<B>,
    function: BbobFn,
    output: &mut Vec<f64>,
    x: &F64_2D<B>,
    xopt: &F64_1D<B>,
    fopt: f64,
) -> bool {
    let mut out: *mut types::futhark_f64_1d = std::ptr::null_mut();

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

pub type RotatedBbobFn = unsafe fn(
    ctx: *mut types::futhark_context,
    y: *mut *mut types::futhark_f64_1d,
    x: *const types::futhark_f64_2d,
    xopt: *const types::futhark_f64_1d,
    fopt: f64,
    R: *const types::futhark_f64_2d,
) -> ::std::os::raw::c_int;

pub fn rotated_bbob<B: Backend>(
    ctx: &Context<B>,
    function: RotatedBbobFn,
    output: &mut Vec<f64>,
    x: &F64_2D<B>,
    xopt: &F64_1D<B>,
    fopt: f64,
    R: &F64_2D<B>,
) -> bool {
    let mut out: *mut types::futhark_f64_1d = std::ptr::null_mut();

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

pub type DoubleRotatedBbobFn = unsafe fn(
    ctx: *mut types::futhark_context,
    y: *mut *mut types::futhark_f64_1d,
    x: *const types::futhark_f64_2d,
    xopt: *const types::futhark_f64_1d,
    fopt: f64,
    R: *const types::futhark_f64_2d,
    Q: *const types::futhark_f64_2d,
) -> ::std::os::raw::c_int;

pub fn double_rotated_bbob<B: Backend>(
    ctx: &Context<B>,
    function: DoubleRotatedBbobFn,
    output: &mut Vec<f64>,
    x: &F64_2D<B>,
    xopt: &F64_1D<B>,
    fopt: f64,
    R: &F64_2D<B>,
    Q: &F64_2D<B>,
) -> bool {
    let mut out: *mut types::futhark_f64_1d = std::ptr::null_mut();

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
