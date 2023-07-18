use crate::{
    backend::{types, Backend},
    Context,
};
use std::slice;

pub struct F64_2D<'c, B: Backend> {
    context: &'c Context<B>,
    pub(crate) inner: *mut types::futhark_f64_2d,
}

impl<'c, B: Backend> F64_2D<'c, B> {
    pub fn new(context: &'c Context<B>, data: &[f64], rows: usize, columns: usize) -> Self {
        assert_eq!(rows * columns, data.len());

        let inner = unsafe {
            B::futhark_new_f64_2d(
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
            let shape = B::futhark_shape_f64_2d(self.context.inner, self.inner);
            slice::from_raw_parts(shape as *const usize, 2)
        }
    }

    pub fn values(&self, out: &mut Vec<f64>) {
        let s = self.shape();
        let len = s[0] * s[1];

        out.reserve(len - out.capacity());
        unsafe {
            B::futhark_values_f64_2d(self.context.inner, self.inner, out.as_mut_ptr());
            out.set_len(len);
        }

        assert!(self.context.sync());
    }
}

impl<B: Backend> Drop for F64_2D<'_, B> {
    fn drop(&mut self) {
        unsafe {
            B::futhark_free_f64_2d(self.context.inner, self.inner);
        }
    }
}
