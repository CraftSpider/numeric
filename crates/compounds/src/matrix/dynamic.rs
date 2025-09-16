use alloc::vec::Vec;
use core::ops::{Index, IndexMut};
use core::ptr::NonNull;
use numeric_traits::class::RealSigned;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DynMatrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> DynMatrix<T> {
    pub fn new(data: Vec<T>, rows: usize, cols: usize) -> DynMatrix<T> {
        assert_eq!(
            data.len(),
            rows * cols,
            "Invalid data length for DynMatrix. Expected length {} ({} by {}), got {}",
            rows * cols,
            rows,
            cols,
            data.len(),
        );

        DynMatrix { data, rows, cols }
    }

    pub(crate) fn as_ptr(&self) -> NonNull<T> {
        NonNull::new(self.data.as_ptr().cast_mut()).unwrap()
    }

    pub(crate) fn as_mut_ptr(&mut self) -> NonNull<T> {
        NonNull::new(self.data.as_mut_ptr()).unwrap()
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn swap_rows(&mut self, a: usize, b: usize) {
        if a == b {
            return;
        }
        let l = usize::min(a, b);
        let r = usize::max(a, b);
        let (start, end) = self.data.split_at_mut(r * self.cols);
        let start = &mut start[l * self.cols..(l + 1) * self.cols];
        let end = &mut end[..self.cols];
        start.swap_with_slice(end);
    }
}

impl<T: RealSigned> DynMatrix<T> {
    gauss_elim!(self, self.rows, self.cols, self.data);
    row_reduce!();
}

impl<T, const ROW: usize, const COL: usize> From<[[T; COL]; ROW]> for DynMatrix<T> {
    fn from(value: [[T; COL]; ROW]) -> Self {
        DynMatrix::new(value.into_iter().flatten().collect(), ROW, COL)
    }
}

impl<T> Index<(usize, usize)> for DynMatrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 * self.cols + index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for DynMatrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 * self.cols + index.1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        let a = DynMatrix::from([
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ]);

        assert_eq!(a[(0, 0)], 1);
        assert_eq!(a[(0, 1)], 2);
        assert_eq!(a[(0, 2)], 3);
        assert_eq!(a[(0, 3)], 4);
        assert_eq!(a[(2, 0)], 9);
        assert_eq!(a[(3, 0)], 13);
        assert_eq!(a[(3, 3)], 16);
    }

    #[test]
    fn test_swap_row() {
        let mut a = DynMatrix::from([
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ]);

        a.swap_rows(0, 2);

        let expected = DynMatrix::from([
            [9, 10, 11, 12],
            [5, 6, 7, 8],
            [1, 2, 3, 4],
            [13, 14, 15, 16],
        ]);
        assert_eq!(a, expected);
    }

    #[test]
    fn test_row_reduce() {
        let a = DynMatrix::from([
            [1., 0., 4., 2.],
            [1., 2., 6., 2.],
            [2., 0., 8., 8.],
            [2., 1., 9., 4.],
        ]);

        let expected = DynMatrix::from([
            [1., 0., 4., 2.],
            [0., 1., 1., 0.],
            [0., 0., 0., 4.],
            [0., 0., 0., 0.],
        ]);
        assert_eq!(a.row_reduce(), expected);
    }
}
