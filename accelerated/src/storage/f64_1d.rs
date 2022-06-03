use crate::{sys, Context};
use std::slice;

pub struct F64_1D<'c> {
    context: &'c Context,
    pub(crate) inner: *mut sys::futhark_f64_1d,
}

impl<'c> F64_1D<'c> {
    pub fn new(context: &'c Context, data: &[f64]) -> Self {
        let inner = unsafe {
            sys::futhark_new_f64_1d(context.inner, data.as_ptr(), data.len().try_into().unwrap())
        };
        assert!(!inner.is_null());
        F64_1D { context, inner }
    }

    pub fn shape(&self) -> &[usize] {
        unsafe {
            let shape = sys::futhark_shape_f64_1d(self.context.inner, self.inner);
            slice::from_raw_parts(shape as *const usize, 1)
        }
    }

    pub fn values(&self, out: &mut Vec<f64>) {
        out.reserve(self.shape()[0] - out.capacity());
        unsafe {
            sys::futhark_values_f64_1d(self.context.inner, self.inner, out.as_mut_ptr());
        }
    }
}

impl Drop for F64_1D<'_> {
    fn drop(&mut self) {
        unsafe {
            sys::futhark_free_f64_1d(self.context.inner, self.inner);
        }
    }
}
