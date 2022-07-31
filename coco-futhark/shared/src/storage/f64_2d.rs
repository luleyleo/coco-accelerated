use crate::{sys, Context};
use std::slice;

pub struct F64_2D<'c> {
    context: &'c Context,
    pub(crate) inner: *mut sys::futhark_f64_2d,
}

impl<'c> F64_2D<'c> {
    pub fn new(context: &'c Context, data: &[f64], rows: usize, columns: usize) -> Self {
        assert_eq!(rows * columns, data.len());

        let inner = unsafe {
            sys::futhark_new_f64_2d(
                context.inner,
                data.as_ptr(),
                rows.try_into().unwrap(),
                columns.try_into().unwrap(),
            )
        };

        assert!(!inner.is_null());
        F64_2D { context, inner }
    }

    pub fn shape(&self) -> &[usize] {
        unsafe {
            let shape = sys::futhark_shape_f64_2d(self.context.inner, self.inner);
            slice::from_raw_parts(shape as *const usize, 2)
        }
    }

    pub fn values(&self, out: &mut Vec<f64>) {
        let s = self.shape();
        let len = s[0] * s[1];

        out.reserve(len - out.capacity());
        unsafe {
            sys::futhark_values_f64_2d(self.context.inner, self.inner, out.as_mut_ptr());
            out.set_len(len);
        }

        assert!(self.context.sync());
    }
}

impl Drop for F64_2D<'_> {
    fn drop(&mut self) {
        unsafe {
            sys::futhark_free_f64_2d(self.context.inner, self.inner);
        }
    }
}
