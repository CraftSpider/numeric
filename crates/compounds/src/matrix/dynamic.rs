use crate::matrix::Matrix;
use crate::vector::Vec2;
use alloc::vec::Vec;
use core::ops::{Index, IndexMut};
use core::ptr::NonNull;
use numeric_traits::class::RealSigned;

/// Arbitrary type matrix with a dynamic number of rows and columns.
///
/// This type will generally be slightly slower than [`Matrix`], but won't explode stack size of
/// large row and column sizes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DynMatrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> DynMatrix<T> {
    /// Create a new matrix from a vector, and the number of rows and columns to split it into.
    /// Note that the length of the data must be exactly equal to `rows * cols`.
    ///
    /// # Panics
    ///
    /// If the length of the data isn't exactly `rows * cols`
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

    /// Get the number of rows in the matrix
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Get the number of columns in the matrix
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Swap two rows of the matrix
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

    /// Swap two columns of the matrix
    pub fn swap_columns(&mut self, a: usize, b: usize) {
        if a == b {
            return;
        }
        for i in 0..self.rows {
            self.data.swap(i * self.cols + a, i * self.cols + b);
        }
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

impl<T, const ROW: usize, const COL: usize> From<Matrix<T, ROW, COL>> for DynMatrix<T> {
    fn from(value: Matrix<T, ROW, COL>) -> Self {
        DynMatrix::from(value.into_arrays())
    }
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct MatrixVecMismatch;

impl<T> TryFrom<Vec<Vec<T>>> for DynMatrix<T> {
    type Error = MatrixVecMismatch;

    fn try_from(values: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        let rows = values.len();
        let cols = values.first().map(Vec::len).unwrap_or(0);
        for row in &values {
            if row.len() != cols {
                return Err(MatrixVecMismatch);
            }
        }
        Ok(DynMatrix::new(
            values.into_iter().flatten().collect(),
            rows,
            cols,
        ))
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

impl<T> Index<Vec2<usize>> for DynMatrix<T> {
    type Output = T;

    fn index(&self, index: Vec2<usize>) -> &Self::Output {
        &self.data[index.y() * self.cols + index.x()]
    }
}

impl<T> IndexMut<Vec2<usize>> for DynMatrix<T> {
    fn index_mut(&mut self, index: Vec2<usize>) -> &mut Self::Output {
        &mut self.data[index.y() * self.cols + index.x()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap() {
        let a = DynMatrix::from([[1, 2, 3], [4, 5, 6]]);

        let mut b = a.clone();
        b.swap_rows(0, 1);
        assert_eq!(b, DynMatrix::from([[4, 5, 6], [1, 2, 3]]));

        let mut b = a.clone();
        b.swap_columns(0, 2);
        assert_eq!(b, DynMatrix::from([[3, 2, 1], [6, 5, 4]]));
    }

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
