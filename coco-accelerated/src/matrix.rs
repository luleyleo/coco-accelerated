#[derive(Debug, Clone, Copy)]
pub struct InputMatrix<'a> {
    data: &'a [f64],
    dimension: usize,
}

impl<'a> InputMatrix<'a> {
    pub fn new(data: &'a [f64], dimension: usize) -> Self {
        assert!(dimension > 0);
        assert!(data.len() % dimension == 0);

        InputMatrix { data, dimension }
    }

    pub fn dimension(&self) -> usize {
        self.dimension
    }

    pub fn data(&self) -> &'a [f64] {
        self.data
    }

    pub fn inputs(&self) -> usize {
        self.data.len() / self.dimension
    }

    pub fn iter_inputs(&self) -> impl Iterator<Item = &[f64]> {
        (0..self.inputs()).map(|i| &self.data[(i * self.dimension)..((i + 1) * self.dimension)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn matrix_items() {
        let data = &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let dimension = 3;
        let input = InputMatrix::new(data, dimension);
        let items = input.iter_inputs().collect::<Vec<&[f64]>>();

        assert_eq!(
            items,
            vec![&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0], &[7.0, 8.0, 9.0]]
        )
    }

    #[test]
    pub fn empty_matrix() {
        let data = &[];
        let dimension = 3;
        let input = InputMatrix::new(data, dimension);

        assert_eq!(input.inputs(), 0);
        assert_eq!(input.dimension(), 3);
        assert_eq!(input.iter_inputs().count(), 0);
    }
}
