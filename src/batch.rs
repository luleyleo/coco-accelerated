#[derive(Debug, Clone, Copy)]
pub struct InputBatch<'a> {
    data: &'a [f64],
    dimension: usize,
}

impl<'a> InputBatch<'a> {
    pub fn new(data: &'a [f64], dimension: usize) -> Self {
        assert!(dimension > 0);
        assert!(data.len() % dimension == 0);

        InputBatch { data, dimension }
    }

    pub fn dimension(&self) -> usize {
        self.dimension
    }

    pub fn data(&self) -> &'a [f64] {
        self.data
    }

    pub fn len(&self) -> usize {
        self.data.len() / self.dimension
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = &[f64]> {
        (0..self.len()).map(|i| &self.data[(i * self.dimension)..((i + 1) * self.dimension)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn iter_items() {
        let data = &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let dimension = 3;
        let input = InputBatch::new(data, dimension);
        let items = input.iter().collect::<Vec<&[f64]>>();

        assert_eq!(
            items,
            vec![&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0], &[7.0, 8.0, 9.0]]
        )
    }

    #[test]
    pub fn empty_batch() {
        let data = &[];
        let dimension = 3;
        let input = InputBatch::new(data, dimension);

        assert!(input.is_empty());
        assert_eq!(input.len(), 0);
        assert_eq!(input.dimension(), 3);
        assert_eq!(input.iter().count(), 0);
    }
}
