use crate::{
    backend::{types, Backend},
    Context,
};
use std::slice;

pub struct F64_1D<'c, B: Backend> {
    context: &'c Context<B>,
    pub(crate) inner: *mut types::futhark_f64_1d,
}

impl<'c, B: Backend> F64_1D<'c, B> {
    pub fn new(context: &'c Context<B>, data: &[f64]) -> Self {
        let inner = unsafe {
            B::futhark_new_f64_1d(context.inner, data.as_ptr(), data.len().try_into().unwrap())
        };
        assert!(!inner.is_null());
        F64_1D { context, inner }
    }

    /// Wraps an existing `f64_1d` array.
    ///
    /// # Safety
    /// `inner` must be a valid futhark array.
    pub unsafe fn from_raw(context: &'c Context<B>, inner: *mut types::futhark_f64_1d) -> Self {
        assert!(!inner.is_null());
        F64_1D { context, inner }
    }

    pub fn shape(&self) -> &[usize] {
        unsafe {
            let shape = B::futhark_shape_f64_1d(self.context.inner, self.inner);
            slice::from_raw_parts(shape as *const usize, 1)
        }
    }

    pub fn values(&self, out: &mut Vec<f64>) {
        out.reserve(self.shape()[0] - out.capacity());
        unsafe {
            B::futhark_values_f64_1d(self.context.inner, self.inner, out.as_mut_ptr());
            out.set_len(self.shape()[0]);
        }

        assert!(self.context.sync());
    }
}

impl<B: Backend> Drop for F64_1D<'_, B> {
    fn drop(&mut self) {
        unsafe {
            B::futhark_free_f64_1d(self.context.inner, self.inner);
        }
    }
}
