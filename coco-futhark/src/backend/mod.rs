#[macro_use]
mod implement;

#[cfg(feature = "c")]
mod c;
#[cfg(feature = "c")]
pub use c::C;

#[cfg(feature = "multicore")]
mod multicore;
#[cfg(feature = "multicore")]
pub use multicore::Multicore;

#[cfg(feature = "opencl")]
mod opencl;
#[cfg(feature = "opencl")]
pub use opencl::OpenCL;

#[cfg(feature = "cuda")]
mod cuda;
#[cfg(feature = "cuda")]
pub use cuda::Cuda;

#[allow(non_camel_case_types)]
pub mod types {
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct futhark_context_config {
        _unused: [u8; 0],
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct futhark_context {
        _unused: [u8; 0],
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct futhark_f64_1d {
        _unused: [u8; 0],
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct futhark_f64_2d {
        _unused: [u8; 0],
    }
}

use types::*;

pub trait Backend {
    unsafe fn futhark_context_config_new() -> *mut futhark_context_config;
    unsafe fn futhark_context_config_free(cfg: *mut futhark_context_config);

    unsafe fn futhark_context_new(cfg: *mut futhark_context_config) -> *mut futhark_context;
    unsafe fn futhark_context_free(cfg: *mut futhark_context);
    unsafe fn futhark_context_sync(ctx: *mut futhark_context) -> ::std::os::raw::c_int;

    unsafe fn futhark_new_f64_1d(
        ctx: *mut futhark_context,
        data: *const f64,
        dim0: i64,
    ) -> *mut futhark_f64_1d;
    unsafe fn futhark_free_f64_1d(
        ctx: *mut futhark_context,
        arr: *mut futhark_f64_1d,
    ) -> ::std::os::raw::c_int;
    unsafe fn futhark_values_f64_1d(
        ctx: *mut futhark_context,
        arr: *mut futhark_f64_1d,
        data: *mut f64,
    ) -> ::std::os::raw::c_int;
    unsafe fn futhark_shape_f64_1d(
        ctx: *mut futhark_context,
        arr: *mut futhark_f64_1d,
    ) -> *const i64;

    unsafe fn futhark_new_f64_2d(
        ctx: *mut futhark_context,
        data: *const f64,
        dim0: i64,
        dim1: i64,
    ) -> *mut futhark_f64_2d;
    unsafe fn futhark_free_f64_2d(
        ctx: *mut futhark_context,
        arr: *mut futhark_f64_2d,
    ) -> ::std::os::raw::c_int;
    unsafe fn futhark_values_f64_2d(
        ctx: *mut futhark_context,
        arr: *mut futhark_f64_2d,
        data: *mut f64,
    ) -> ::std::os::raw::c_int;
    unsafe fn futhark_shape_f64_2d(
        ctx: *mut futhark_context,
        arr: *mut futhark_f64_2d,
    ) -> *const i64;

    unsafe fn futhark_entry_attractive_sector(
        ctx: *mut futhark_context,
        out0: *mut *mut futhark_f64_1d,
        in0: *const futhark_f64_2d,
        in1: *const futhark_f64_1d,
        in2: f64,
        in3: *const futhark_f64_2d,
    ) -> ::std::os::raw::c_int;
    unsafe fn futhark_entry_bent_cigar(
        ctx: *mut futhark_context,
        out0: *mut *mut futhark_f64_1d,
        in0: *const futhark_f64_2d,
        in1: *const futhark_f64_1d,
        in2: f64,
        in3: *const futhark_f64_2d,
    ) -> ::std::os::raw::c_int;
    unsafe fn futhark_entry_bueche_rastrigin(
        ctx: *mut futhark_context,
        out0: *mut *mut futhark_f64_1d,
        in0: *const futhark_f64_2d,
        in1: *const futhark_f64_1d,
        in2: f64,
    ) -> ::std::os::raw::c_int;
    unsafe fn futhark_entry_different_powers(
        ctx: *mut futhark_context,
        out0: *mut *mut futhark_f64_1d,
        in0: *const futhark_f64_2d,
        in1: *const futhark_f64_1d,
        in2: f64,
        in3: *const futhark_f64_2d,
    ) -> ::std::os::raw::c_int;
    unsafe fn futhark_entry_discus(
        ctx: *mut futhark_context,
        out0: *mut *mut futhark_f64_1d,
        in0: *const futhark_f64_2d,
        in1: *const futhark_f64_1d,
        in2: f64,
        in3: *const futhark_f64_2d,
    ) -> ::std::os::raw::c_int;
    unsafe fn futhark_entry_ellipsoidal(
        ctx: *mut futhark_context,
        out0: *mut *mut futhark_f64_1d,
        in0: *const futhark_f64_2d,
        in1: *const futhark_f64_1d,
        in2: f64,
    ) -> ::std::os::raw::c_int;
    unsafe fn futhark_entry_ellipsoidal_rotated(
        ctx: *mut futhark_context,
        out0: *mut *mut futhark_f64_1d,
        in0: *const futhark_f64_2d,
        in1: *const futhark_f64_1d,
        in2: f64,
        in3: *const futhark_f64_2d,
    ) -> ::std::os::raw::c_int;
    unsafe fn futhark_entry_gallagher(
        ctx: *mut futhark_context,
        out0: *mut *mut futhark_f64_1d,
        in0: *const futhark_f64_2d,
        in1: *const futhark_f64_2d,
        in2: *const futhark_f64_1d,
        in3: *const futhark_f64_2d,
        in4: f64,
        in5: *const futhark_f64_2d,
    ) -> ::std::os::raw::c_int;
    unsafe fn futhark_entry_griewank_rosenbrock(
        ctx: *mut futhark_context,
        out0: *mut *mut futhark_f64_1d,
        in0: *const futhark_f64_2d,
        in1: f64,
        in2: *const futhark_f64_2d,
    ) -> ::std::os::raw::c_int;
    unsafe fn futhark_entry_katsuura(
        ctx: *mut futhark_context,
        out0: *mut *mut futhark_f64_1d,
        in0: *const futhark_f64_2d,
        in1: *const futhark_f64_1d,
        in2: f64,
        in3: *const futhark_f64_2d,
        in4: *const futhark_f64_2d,
    ) -> ::std::os::raw::c_int;
    unsafe fn futhark_entry_linear_slope(
        ctx: *mut futhark_context,
        out0: *mut *mut futhark_f64_1d,
        in0: *const futhark_f64_2d,
        in1: *const futhark_f64_1d,
        in2: f64,
    ) -> ::std::os::raw::c_int;
    unsafe fn futhark_entry_lunacek(
        ctx: *mut futhark_context,
        out0: *mut *mut futhark_f64_1d,
        in0: *const futhark_f64_2d,
        in1: *const futhark_f64_1d,
        in2: f64,
        in3: *const futhark_f64_2d,
    ) -> ::std::os::raw::c_int;
    unsafe fn futhark_entry_rastrigin(
        ctx: *mut futhark_context,
        out0: *mut *mut futhark_f64_1d,
        in0: *const futhark_f64_2d,
        in1: *const futhark_f64_1d,
        in2: f64,
    ) -> ::std::os::raw::c_int;
    unsafe fn futhark_entry_rastrigin_rotated(
        ctx: *mut futhark_context,
        out0: *mut *mut futhark_f64_1d,
        in0: *const futhark_f64_2d,
        in1: *const futhark_f64_1d,
        in2: f64,
        in3: *const futhark_f64_2d,
        in4: *const futhark_f64_2d,
    ) -> ::std::os::raw::c_int;
    unsafe fn futhark_entry_rosenbrock(
        ctx: *mut futhark_context,
        out0: *mut *mut futhark_f64_1d,
        in0: *const futhark_f64_2d,
        in1: *const futhark_f64_1d,
        in2: f64,
    ) -> ::std::os::raw::c_int;
    unsafe fn futhark_entry_rosenbrock_rotated(
        ctx: *mut futhark_context,
        out0: *mut *mut futhark_f64_1d,
        in0: *const futhark_f64_2d,
        in1: f64,
        in2: *const futhark_f64_2d,
    ) -> ::std::os::raw::c_int;
    unsafe fn futhark_entry_schaffers_f7(
        ctx: *mut futhark_context,
        out0: *mut *mut futhark_f64_1d,
        in0: *const futhark_f64_2d,
        in1: *const futhark_f64_1d,
        in2: f64,
        in3: *const futhark_f64_2d,
        in4: *const futhark_f64_2d,
    ) -> ::std::os::raw::c_int;
    unsafe fn futhark_entry_schaffers_f7_ill(
        ctx: *mut futhark_context,
        out0: *mut *mut futhark_f64_1d,
        in0: *const futhark_f64_2d,
        in1: *const futhark_f64_1d,
        in2: f64,
        in3: *const futhark_f64_2d,
        in4: *const futhark_f64_2d,
    ) -> ::std::os::raw::c_int;
    unsafe fn futhark_entry_schwefel(
        ctx: *mut futhark_context,
        out0: *mut *mut futhark_f64_1d,
        in0: *const futhark_f64_2d,
        in1: *const futhark_f64_1d,
        in2: f64,
    ) -> ::std::os::raw::c_int;
    unsafe fn futhark_entry_sharp_ridge(
        ctx: *mut futhark_context,
        out0: *mut *mut futhark_f64_1d,
        in0: *const futhark_f64_2d,
        in1: *const futhark_f64_1d,
        in2: f64,
        in3: *const futhark_f64_2d,
    ) -> ::std::os::raw::c_int;
    unsafe fn futhark_entry_sphere(
        ctx: *mut futhark_context,
        out0: *mut *mut futhark_f64_1d,
        in0: *const futhark_f64_2d,
        in1: *const futhark_f64_1d,
        in2: f64,
    ) -> ::std::os::raw::c_int;
    unsafe fn futhark_entry_step_ellipsoidal(
        ctx: *mut futhark_context,
        out0: *mut *mut futhark_f64_1d,
        in0: *const futhark_f64_2d,
        in1: *const futhark_f64_1d,
        in2: f64,
        in3: *const futhark_f64_2d,
        in4: *const futhark_f64_2d,
    ) -> ::std::os::raw::c_int;
    unsafe fn futhark_entry_weierstrass(
        ctx: *mut futhark_context,
        out0: *mut *mut futhark_f64_1d,
        in0: *const futhark_f64_2d,
        in1: *const futhark_f64_1d,
        in2: f64,
        in3: *const futhark_f64_2d,
        in4: *const futhark_f64_2d,
    ) -> ::std::os::raw::c_int;
}
