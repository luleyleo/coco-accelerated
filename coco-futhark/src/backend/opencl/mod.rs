use crate::backend::{types, Backend};

mod sys;

pub struct OpenCL;

impl Backend for OpenCL {
    unsafe fn futhark_context_config_new() -> *mut types::futhark_context_config {
        sys::futhark_context_config_new() as *mut types::futhark_context_config
    }

    unsafe fn futhark_context_config_free(cfg: *mut types::futhark_context_config) {
        sys::futhark_context_config_free(cfg as *mut sys::futhark_context_config);
    }

    unsafe fn futhark_context_new(
        cfg: *mut types::futhark_context_config,
    ) -> *mut types::futhark_context {
        sys::futhark_context_new(cfg as *mut sys::futhark_context_config)
            as *mut types::futhark_context
    }

    unsafe fn futhark_context_free(cfg: *mut types::futhark_context) {
        sys::futhark_context_free(cfg as *mut sys::futhark_context);
    }

    unsafe fn futhark_context_sync(ctx: *mut types::futhark_context) -> ::std::os::raw::c_int {
        sys::futhark_context_sync(ctx as *mut sys::futhark_context)
    }

    unsafe fn futhark_new_f64_1d(
        ctx: *mut types::futhark_context,
        data: *const f64,
        dim0: i64,
    ) -> *mut types::futhark_f64_1d {
        sys::futhark_new_f64_1d(ctx as *mut sys::futhark_context, data, dim0)
            as *mut types::futhark_f64_1d
    }

    unsafe fn futhark_free_f64_1d(
        ctx: *mut types::futhark_context,
        arr: *mut types::futhark_f64_1d,
    ) -> std::os::raw::c_int {
        sys::futhark_free_f64_1d(
            ctx as *mut sys::futhark_context,
            arr as *mut sys::futhark_f64_1d,
        )
    }

    unsafe fn futhark_values_f64_1d(
        ctx: *mut types::futhark_context,
        arr: *mut types::futhark_f64_1d,
        data: *mut f64,
    ) -> std::os::raw::c_int {
        sys::futhark_values_f64_1d(
            ctx as *mut sys::futhark_context,
            arr as *mut sys::futhark_f64_1d,
            data,
        )
    }

    unsafe fn futhark_shape_f64_1d(
        ctx: *mut types::futhark_context,
        arr: *mut types::futhark_f64_1d,
    ) -> *const i64 {
        sys::futhark_shape_f64_1d(
            ctx as *mut sys::futhark_context,
            arr as *mut sys::futhark_f64_1d,
        )
    }

    unsafe fn futhark_new_f64_2d(
        ctx: *mut types::futhark_context,
        data: *const f64,
        dim0: i64,
        dim1: i64,
    ) -> *mut types::futhark_f64_2d {
        sys::futhark_new_f64_2d(ctx as *mut sys::futhark_context, data, dim0, dim1)
            as *mut types::futhark_f64_2d
    }

    unsafe fn futhark_free_f64_2d(
        ctx: *mut types::futhark_context,
        arr: *mut types::futhark_f64_2d,
    ) -> std::os::raw::c_int {
        sys::futhark_free_f64_2d(
            ctx as *mut sys::futhark_context,
            arr as *mut sys::futhark_f64_2d,
        )
    }

    unsafe fn futhark_values_f64_2d(
        ctx: *mut types::futhark_context,
        arr: *mut types::futhark_f64_2d,
        data: *mut f64,
    ) -> std::os::raw::c_int {
        sys::futhark_values_f64_2d(
            ctx as *mut sys::futhark_context,
            arr as *mut sys::futhark_f64_2d,
            data,
        )
    }

    unsafe fn futhark_shape_f64_2d(
        ctx: *mut types::futhark_context,
        arr: *mut types::futhark_f64_2d,
    ) -> *const i64 {
        sys::futhark_shape_f64_2d(
            ctx as *mut sys::futhark_context,
            arr as *mut sys::futhark_f64_2d,
        )
    }

    unsafe fn futhark_entry_attractive_sector(
        ctx: *mut types::futhark_context,
        out0: *mut *mut types::futhark_f64_1d,
        in0: *const types::futhark_f64_2d,
        in1: *const types::futhark_f64_1d,
        in2: f64,
        in3: *const types::futhark_f64_2d,
    ) -> std::os::raw::c_int {
        sys::futhark_entry_attractive_sector(
            ctx as *mut sys::futhark_context,
            out0 as *mut *mut sys::futhark_f64_1d,
            in0 as *const sys::futhark_f64_2d,
            in1 as *const sys::futhark_f64_1d,
            in2,
            in3 as *const sys::futhark_f64_2d,
        )
    }

    unsafe fn futhark_entry_bent_cigar(
        ctx: *mut types::futhark_context,
        out0: *mut *mut types::futhark_f64_1d,
        in0: *const types::futhark_f64_2d,
        in1: *const types::futhark_f64_1d,
        in2: f64,
        in3: *const types::futhark_f64_2d,
    ) -> std::os::raw::c_int {
        sys::futhark_entry_bent_cigar(
            ctx as *mut sys::futhark_context,
            out0 as *mut *mut sys::futhark_f64_1d,
            in0 as *const sys::futhark_f64_2d,
            in1 as *const sys::futhark_f64_1d,
            in2 as f64,
            in3 as *const sys::futhark_f64_2d,
        )
    }

    unsafe fn futhark_entry_bueche_rastrigin(
        ctx: *mut types::futhark_context,
        out0: *mut *mut types::futhark_f64_1d,
        in0: *const types::futhark_f64_2d,
        in1: *const types::futhark_f64_1d,
        in2: f64,
    ) -> std::os::raw::c_int {
        sys::futhark_entry_bueche_rastrigin(
            ctx as *mut sys::futhark_context,
            out0 as *mut *mut sys::futhark_f64_1d,
            in0 as *const sys::futhark_f64_2d,
            in1 as *const sys::futhark_f64_1d,
            in2 as f64,
        )
    }

    unsafe fn futhark_entry_different_powers(
        ctx: *mut types::futhark_context,
        out0: *mut *mut types::futhark_f64_1d,
        in0: *const types::futhark_f64_2d,
        in1: *const types::futhark_f64_1d,
        in2: f64,
        in3: *const types::futhark_f64_2d,
    ) -> std::os::raw::c_int {
        sys::futhark_entry_different_powers(
            ctx as *mut sys::futhark_context,
            out0 as *mut *mut sys::futhark_f64_1d,
            in0 as *const sys::futhark_f64_2d,
            in1 as *const sys::futhark_f64_1d,
            in2 as f64,
            in3 as *const sys::futhark_f64_2d,
        )
    }

    unsafe fn futhark_entry_discus(
        ctx: *mut types::futhark_context,
        out0: *mut *mut types::futhark_f64_1d,
        in0: *const types::futhark_f64_2d,
        in1: *const types::futhark_f64_1d,
        in2: f64,
        in3: *const types::futhark_f64_2d,
    ) -> std::os::raw::c_int {
        sys::futhark_entry_discus(
            ctx as *mut sys::futhark_context,
            out0 as *mut *mut sys::futhark_f64_1d,
            in0 as *const sys::futhark_f64_2d,
            in1 as *const sys::futhark_f64_1d,
            in2 as f64,
            in3 as *const sys::futhark_f64_2d,
        )
    }

    unsafe fn futhark_entry_ellipsoidal(
        ctx: *mut types::futhark_context,
        out0: *mut *mut types::futhark_f64_1d,
        in0: *const types::futhark_f64_2d,
        in1: *const types::futhark_f64_1d,
        in2: f64,
    ) -> std::os::raw::c_int {
        sys::futhark_entry_ellipsoidal(
            ctx as *mut sys::futhark_context,
            out0 as *mut *mut sys::futhark_f64_1d,
            in0 as *const sys::futhark_f64_2d,
            in1 as *const sys::futhark_f64_1d,
            in2 as f64,
        )
    }

    unsafe fn futhark_entry_ellipsoidal_rotated(
        ctx: *mut types::futhark_context,
        out0: *mut *mut types::futhark_f64_1d,
        in0: *const types::futhark_f64_2d,
        in1: *const types::futhark_f64_1d,
        in2: f64,
        in3: *const types::futhark_f64_2d,
    ) -> std::os::raw::c_int {
        sys::futhark_entry_ellipsoidal_rotated(
            ctx as *mut sys::futhark_context,
            out0 as *mut *mut sys::futhark_f64_1d,
            in0 as *const sys::futhark_f64_2d,
            in1 as *const sys::futhark_f64_1d,
            in2 as f64,
            in3 as *const sys::futhark_f64_2d,
        )
    }

    unsafe fn futhark_entry_gallagher(
        ctx: *mut types::futhark_context,
        out0: *mut *mut types::futhark_f64_1d,
        in0: *const types::futhark_f64_2d,
        in1: *const types::futhark_f64_2d,
        in2: *const types::futhark_f64_1d,
        in3: *const types::futhark_f64_2d,
        in4: f64,
        in5: *const types::futhark_f64_2d,
    ) -> std::os::raw::c_int {
        sys::futhark_entry_gallagher(
            ctx as *mut sys::futhark_context,
            out0 as *mut *mut sys::futhark_f64_1d,
            in0 as *const sys::futhark_f64_2d,
            in1 as *const sys::futhark_f64_2d,
            in2 as *const sys::futhark_f64_1d,
            in3 as *const sys::futhark_f64_2d,
            in4 as f64,
            in5 as *const sys::futhark_f64_2d,
        )
    }

    unsafe fn futhark_entry_griewank_rosenbrock(
        ctx: *mut types::futhark_context,
        out0: *mut *mut types::futhark_f64_1d,
        in0: *const types::futhark_f64_2d,
        in1: f64,
        in2: *const types::futhark_f64_2d,
    ) -> std::os::raw::c_int {
        sys::futhark_entry_griewank_rosenbrock(
            ctx as *mut sys::futhark_context,
            out0 as *mut *mut sys::futhark_f64_1d,
            in0 as *const sys::futhark_f64_2d,
            in1 as f64,
            in2 as *const sys::futhark_f64_2d,
        )
    }

    unsafe fn futhark_entry_katsuura(
        ctx: *mut types::futhark_context,
        out0: *mut *mut types::futhark_f64_1d,
        in0: *const types::futhark_f64_2d,
        in1: *const types::futhark_f64_1d,
        in2: f64,
        in3: *const types::futhark_f64_2d,
        in4: *const types::futhark_f64_2d,
    ) -> std::os::raw::c_int {
        sys::futhark_entry_katsuura(
            ctx as *mut sys::futhark_context,
            out0 as *mut *mut sys::futhark_f64_1d,
            in0 as *const sys::futhark_f64_2d,
            in1 as *const sys::futhark_f64_1d,
            in2 as f64,
            in3 as *const sys::futhark_f64_2d,
            in4 as *const sys::futhark_f64_2d,
        )
    }

    unsafe fn futhark_entry_linear_slope(
        ctx: *mut types::futhark_context,
        out0: *mut *mut types::futhark_f64_1d,
        in0: *const types::futhark_f64_2d,
        in1: *const types::futhark_f64_1d,
        in2: f64,
    ) -> std::os::raw::c_int {
        sys::futhark_entry_linear_slope(
            ctx as *mut sys::futhark_context,
            out0 as *mut *mut sys::futhark_f64_1d,
            in0 as *const sys::futhark_f64_2d,
            in1 as *const sys::futhark_f64_1d,
            in2 as f64,
        )
    }

    unsafe fn futhark_entry_lunacek(
        ctx: *mut types::futhark_context,
        out0: *mut *mut types::futhark_f64_1d,
        in0: *const types::futhark_f64_2d,
        in1: *const types::futhark_f64_1d,
        in2: f64,
        in3: *const types::futhark_f64_2d,
    ) -> std::os::raw::c_int {
        sys::futhark_entry_lunacek(
            ctx as *mut sys::futhark_context,
            out0 as *mut *mut sys::futhark_f64_1d,
            in0 as *const sys::futhark_f64_2d,
            in1 as *const sys::futhark_f64_1d,
            in2 as f64,
            in3 as *const sys::futhark_f64_2d,
        )
    }

    unsafe fn futhark_entry_rastrigin(
        ctx: *mut types::futhark_context,
        out0: *mut *mut types::futhark_f64_1d,
        in0: *const types::futhark_f64_2d,
        in1: *const types::futhark_f64_1d,
        in2: f64,
    ) -> std::os::raw::c_int {
        sys::futhark_entry_rastrigin(
            ctx as *mut sys::futhark_context,
            out0 as *mut *mut sys::futhark_f64_1d,
            in0 as *const sys::futhark_f64_2d,
            in1 as *const sys::futhark_f64_1d,
            in2 as f64,
        )
    }

    unsafe fn futhark_entry_rastrigin_rotated(
        ctx: *mut types::futhark_context,
        out0: *mut *mut types::futhark_f64_1d,
        in0: *const types::futhark_f64_2d,
        in1: *const types::futhark_f64_1d,
        in2: f64,
        in3: *const types::futhark_f64_2d,
        in4: *const types::futhark_f64_2d,
    ) -> std::os::raw::c_int {
        sys::futhark_entry_rastrigin_rotated(
            ctx as *mut sys::futhark_context,
            out0 as *mut *mut sys::futhark_f64_1d,
            in0 as *const sys::futhark_f64_2d,
            in1 as *const sys::futhark_f64_1d,
            in2 as f64,
            in3 as *const sys::futhark_f64_2d,
            in4 as *const sys::futhark_f64_2d,
        )
    }

    unsafe fn futhark_entry_rosenbrock(
        ctx: *mut types::futhark_context,
        out0: *mut *mut types::futhark_f64_1d,
        in0: *const types::futhark_f64_2d,
        in1: *const types::futhark_f64_1d,
        in2: f64,
    ) -> std::os::raw::c_int {
        sys::futhark_entry_rosenbrock(
            ctx as *mut sys::futhark_context,
            out0 as *mut *mut sys::futhark_f64_1d,
            in0 as *const sys::futhark_f64_2d,
            in1 as *const sys::futhark_f64_1d,
            in2 as f64,
        )
    }

    unsafe fn futhark_entry_rosenbrock_rotated(
        ctx: *mut types::futhark_context,
        out0: *mut *mut types::futhark_f64_1d,
        in0: *const types::futhark_f64_2d,
        in1: f64,
        in2: *const types::futhark_f64_2d,
    ) -> std::os::raw::c_int {
        sys::futhark_entry_rosenbrock_rotated(
            ctx as *mut sys::futhark_context,
            out0 as *mut *mut sys::futhark_f64_1d,
            in0 as *const sys::futhark_f64_2d,
            in1 as f64,
            in2 as *const sys::futhark_f64_2d,
        )
    }

    unsafe fn futhark_entry_schaffers_f7(
        ctx: *mut types::futhark_context,
        out0: *mut *mut types::futhark_f64_1d,
        in0: *const types::futhark_f64_2d,
        in1: *const types::futhark_f64_1d,
        in2: f64,
        in3: *const types::futhark_f64_2d,
        in4: *const types::futhark_f64_2d,
    ) -> std::os::raw::c_int {
        sys::futhark_entry_schaffers_f7(
            ctx as *mut sys::futhark_context,
            out0 as *mut *mut sys::futhark_f64_1d,
            in0 as *const sys::futhark_f64_2d,
            in1 as *const sys::futhark_f64_1d,
            in2 as f64,
            in3 as *const sys::futhark_f64_2d,
            in4 as *const sys::futhark_f64_2d,
        )
    }

    unsafe fn futhark_entry_schaffers_f7_ill(
        ctx: *mut types::futhark_context,
        out0: *mut *mut types::futhark_f64_1d,
        in0: *const types::futhark_f64_2d,
        in1: *const types::futhark_f64_1d,
        in2: f64,
        in3: *const types::futhark_f64_2d,
        in4: *const types::futhark_f64_2d,
    ) -> std::os::raw::c_int {
        sys::futhark_entry_schaffers_f7_ill(
            ctx as *mut sys::futhark_context,
            out0 as *mut *mut sys::futhark_f64_1d,
            in0 as *const sys::futhark_f64_2d,
            in1 as *const sys::futhark_f64_1d,
            in2 as f64,
            in3 as *const sys::futhark_f64_2d,
            in4 as *const sys::futhark_f64_2d,
        )
    }

    unsafe fn futhark_entry_schwefel(
        ctx: *mut types::futhark_context,
        out0: *mut *mut types::futhark_f64_1d,
        in0: *const types::futhark_f64_2d,
        in1: *const types::futhark_f64_1d,
        in2: f64,
    ) -> std::os::raw::c_int {
        sys::futhark_entry_schwefel(
            ctx as *mut sys::futhark_context,
            out0 as *mut *mut sys::futhark_f64_1d,
            in0 as *const sys::futhark_f64_2d,
            in1 as *const sys::futhark_f64_1d,
            in2 as f64,
        )
    }

    unsafe fn futhark_entry_sharp_ridge(
        ctx: *mut types::futhark_context,
        out0: *mut *mut types::futhark_f64_1d,
        in0: *const types::futhark_f64_2d,
        in1: *const types::futhark_f64_1d,
        in2: f64,
        in3: *const types::futhark_f64_2d,
    ) -> std::os::raw::c_int {
        sys::futhark_entry_sharp_ridge(
            ctx as *mut sys::futhark_context,
            out0 as *mut *mut sys::futhark_f64_1d,
            in0 as *const sys::futhark_f64_2d,
            in1 as *const sys::futhark_f64_1d,
            in2 as f64,
            in3 as *const sys::futhark_f64_2d,
        )
    }

    unsafe fn futhark_entry_sphere(
        ctx: *mut types::futhark_context,
        out0: *mut *mut types::futhark_f64_1d,
        in0: *const types::futhark_f64_2d,
        in1: *const types::futhark_f64_1d,
        in2: f64,
    ) -> std::os::raw::c_int {
        sys::futhark_entry_sphere(
            ctx as *mut sys::futhark_context,
            out0 as *mut *mut sys::futhark_f64_1d,
            in0 as *const sys::futhark_f64_2d,
            in1 as *const sys::futhark_f64_1d,
            in2 as f64,
        )
    }

    unsafe fn futhark_entry_step_ellipsoidal(
        ctx: *mut types::futhark_context,
        out0: *mut *mut types::futhark_f64_1d,
        in0: *const types::futhark_f64_2d,
        in1: *const types::futhark_f64_1d,
        in2: f64,
        in3: *const types::futhark_f64_2d,
        in4: *const types::futhark_f64_2d,
    ) -> std::os::raw::c_int {
        sys::futhark_entry_step_ellipsoidal(
            ctx as *mut sys::futhark_context,
            out0 as *mut *mut sys::futhark_f64_1d,
            in0 as *const sys::futhark_f64_2d,
            in1 as *const sys::futhark_f64_1d,
            in2 as f64,
            in3 as *const sys::futhark_f64_2d,
            in4 as *const sys::futhark_f64_2d,
        )
    }

    unsafe fn futhark_entry_weierstrass(
        ctx: *mut types::futhark_context,
        out0: *mut *mut types::futhark_f64_1d,
        in0: *const types::futhark_f64_2d,
        in1: *const types::futhark_f64_1d,
        in2: f64,
        in3: *const types::futhark_f64_2d,
        in4: *const types::futhark_f64_2d,
    ) -> std::os::raw::c_int {
        sys::futhark_entry_weierstrass(
            ctx as *mut sys::futhark_context,
            out0 as *mut *mut sys::futhark_f64_1d,
            in0 as *const sys::futhark_f64_2d,
            in1 as *const sys::futhark_f64_1d,
            in2 as f64,
            in3 as *const sys::futhark_f64_2d,
            in4 as *const sys::futhark_f64_2d,
        )
    }
}
