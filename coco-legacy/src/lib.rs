//! BBOB legacy code from 2009
//!
//! This reuses the C legacy code to guarantee the same behaviour
//! as the official coco benchmark suite.

type CocoMatrix = *const *mut f64;

pub struct Matrix {
    pub dimension: usize,
    pub data: Box<[f64]>,
}

extern "C" {
    fn bbob2009_unif(r: *mut f64, dim: isize, inseed: i64);
    fn bbob2009_compute_xopt(xopt: *mut f64, seed: i64, dim: isize);
    fn bbob2009_compute_fopt(function: isize, instance: isize) -> f64;
    fn bbob2009_compute_rotation(matrix: CocoMatrix, seed: i64, dim: isize);
}

pub fn compute_unif(seed: usize, dimension: usize) -> Vec<f64> {
    let mut unif = vec![0.0; dimension];

    unsafe {
        bbob2009_unif(unif.as_mut_ptr(), dimension as isize, seed as i64);
    }

    unif
}

pub fn compute_xopt(seed: usize, dimension: usize) -> Vec<f64> {
    let mut xopt = vec![0.0; dimension];

    unsafe {
        bbob2009_compute_xopt(xopt.as_mut_ptr(), seed as i64, dimension as isize);
    }

    xopt
}

pub fn compute_fopt(function: usize, instance: usize) -> f64 {
    unsafe { bbob2009_compute_fopt(function as isize, instance as isize) }
}

pub fn compute_rotation(seed: usize, dimension: usize) -> Matrix {
    let mut data = vec![0.0; dimension * dimension].into_boxed_slice();

    unsafe {
        let coco_matrix = (0..dimension)
            .map(|i| data.as_mut_ptr().add(i * dimension))
            .collect::<Vec<_>>();

        bbob2009_compute_rotation(coco_matrix.as_ptr(), seed as i64, dimension as isize);
    }

    Matrix { dimension, data }
}

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;

    #[test]
    fn compute_xopt() {
        let rseed = 4 + 10000 * 2;
        let x = super::compute_xopt(rseed, 5);
        let xe = vec![-3.123200, -1.584800, -3.537600, 1.694400, 3.956000];
        assert_float_eq!(&x, &xe, abs_all <= 0.0001);
    }

    #[test]
    fn compute_fopt() {
        let y = super::compute_fopt(4, 2);
        assert_float_eq!(y, 77.66, abs <= 0.0);
    }
}
