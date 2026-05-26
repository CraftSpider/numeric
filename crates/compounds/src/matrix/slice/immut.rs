use crate::matrix::slice::size::PackedSize;
#[cfg(feature = "alloc")]
use crate::matrix::DynMatrix;
use crate::matrix::{
    bound_end, bound_start, index_check, range_check, slice, MatMut, Matrix, MatrixLike,
};
use crate::vector::Vec2;
use core::fmt;
use core::marker::PhantomData;
use core::ops::{Index, RangeBounds};
use core::ptr::NonNull;
use numeric_utils::static_assert;

static_assert!(size_of::<MatSlice<'_, f32>>() == size_of::<Option<MatSlice<'_, f32>>>());

/// Matrix sub-slice. A view into a matrix, the same way a normal `[T]` slice is a view into
/// a contiguous array.
///
/// Immutable variant of [`MatMut`].
#[repr(C)]
pub struct MatSlice<'a, T> {
    data: NonNull<T>,
    size: PackedSize,
    _phantom: PhantomData<&'a [T]>,
}

impl<'a, T> MatSlice<'a, T> {
    pub(crate) fn new_raw(data: NonNull<T>, size: PackedSize) -> MatSlice<'a, T> {
        MatSlice {
            data,
            size,
            _phantom: PhantomData,
        }
    }

    /// Construct a matrix slice from raw parts
    ///
    /// # Safety
    ///
    /// The provided pointer must point to a valid position in a matrix in row-major order.
    ///
    /// It must be safe to read contiguous ranges of length `cols` every `stride` elements,
    /// `rows` times. Items in between these ranges need not be valid for reads.
    pub unsafe fn from_raw_parts(
        data: *const T,
        rows: usize,
        cols: usize,
        stride: usize,
    ) -> MatSlice<'a, T> {
        MatSlice {
            data: NonNull::new_unchecked(data.cast_mut()),
            size: PackedSize::new(rows, cols, stride),
            _phantom: PhantomData,
        }
    }

    /// Convert this matrix slice into raw parts
    pub fn into_raw_parts(self) -> (*const T, usize, usize, usize) {
        let (rows, cols, stride) = self.size.unpack();
        (self.data.as_ptr(), rows, cols, stride)
    }

    /// Get the number of rows in the matrix
    pub fn rows(&self) -> usize {
        self.size.len()
    }

    /// Get the number of columns in the matrix
    pub fn cols(&self) -> usize {
        self.size.depth()
    }

    pub(crate) fn stride(&self) -> usize {
        self.size.stride()
    }

    pub(crate) fn ptr(&self) -> NonNull<T> {
        self.data
    }

    /// Get a sub-slice of this matrix slice, by rows and columns.
    ///
    /// # Panics
    ///
    /// If the provided ranges are out of bounds for this slice. For a non-panicking version, use
    /// [`Self::try_slice`].
    pub fn slice(
        &self,
        rows: impl RangeBounds<usize>,
        cols: impl RangeBounds<usize>,
    ) -> MatSlice<'a, T> {
        let stride = self.size.stride();

        let row_start = bound_start(rows.start_bound());
        let row_end = bound_end(rows.end_bound()).unwrap_or(self.rows());

        let col_start = bound_start(cols.start_bound());
        let col_end = bound_end(cols.end_bound()).unwrap_or(self.cols());

        range_check(
            (self.rows(), self.cols()),
            (row_start, row_end),
            (col_start, col_end),
        );

        // SAFETY: Row and column starts have been verified in-bounds of our pointer range
        let ptr = unsafe { self.data.add(col_start).add(row_start * stride) };
        // SAFETY: Internal pointer and stride guaranteed valid.
        //         Rows and columns ranges validated in-bounds.
        unsafe {
            MatSlice::from_raw_parts(
                ptr.as_ptr(),
                row_end - row_start,
                col_end - col_start,
                stride,
            )
        }
    }

    /// Iterate over rows of this matrix slice.
    pub fn row_iter(&self) -> slice::RowIter<'a, T> {
        slice::RowIter::new(*self)
    }

    /// Iterate over columns of this matrix slice.
    pub fn column_iter(&self) -> slice::ColumnIter<'a, T> {
        slice::ColumnIter::new(*self)
    }

    /// Iterate over all elements in this matrix slice in row-major order.
    pub fn iter_row_major(&self) -> slice::RowMajorIter<'a, T> {
        slice::RowMajorIter::new(*self)
    }

    /// Iterate over all elements in this matrix slice in column-major order.
    pub fn iter_column_major(&self) -> slice::ColumnMajorIter<'a, T> {
        slice::ColumnMajorIter::new(*self)
    }
}

impl<T> Clone for MatSlice<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for MatSlice<'_, T> {}

impl<T: fmt::Debug> fmt::Debug for MatSlice<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        let mut first = true;
        for i in 0..self.rows() {
            if !first {
                write!(f, ", ")?;
            } else {
                first = false;
            }

            let mut first2 = true;
            write!(f, "[")?;
            for j in 0..self.cols() {
                if !first2 {
                    write!(f, ", ")?;
                } else {
                    first2 = false;
                }

                write!(f, "{:?}", self[(i, j)])?;
            }
            write!(f, "]")?;
        }
        write!(f, "]")
    }
}

impl<'a, T> Index<(usize, usize)> for MatSlice<'a, T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (rows, cols, stride) = self.size.unpack();
        index_check((rows, cols), index);
        // SAFETY: Internal pointer guaranteed valid for reads in checked range
        unsafe { self.data.add(index.0 * stride + index.1).as_ref() }
    }
}

impl<'a, T> Index<Vec2<usize>> for MatSlice<'a, T> {
    type Output = T;

    fn index(&self, index: Vec2<usize>) -> &Self::Output {
        &self[(*index.x(), *index.y())]
    }
}

impl<'a, T> From<MatMut<'a, T>> for MatSlice<'a, T> {
    fn from(value: MatMut<'a, T>) -> Self {
        Self::new_raw(value.ptr(), value.size())
    }
}

impl<'a, T, const ROW: usize, const COL: usize> From<&'a Matrix<T, ROW, COL>> for MatSlice<'a, T> {
    fn from(value: &'a Matrix<T, ROW, COL>) -> Self {
        // SAFETY: Arguments trivially valid by definition
        unsafe { MatSlice::from_raw_parts(value.as_ptr(), ROW, COL, COL) }
    }
}

#[cfg(feature = "alloc")]
impl<'a, T> From<&'a DynMatrix<T>> for MatSlice<'a, T> {
    fn from(value: &'a DynMatrix<T>) -> Self {
        // SAFETY: Arguments trivially valid by definition
        unsafe {
            MatSlice::from_raw_parts(value.as_ptr(), value.rows(), value.cols(), value.cols())
        }
    }
}

impl<T: PartialEq> PartialEq for MatSlice<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        if self.rows() != other.rows() || self.cols() != other.cols() {
            false
        } else {
            // TODO: Use iterator over elements
            for i in 0..self.rows() {
                for j in 0..self.cols() {
                    if self[(i, j)] != other[(i, j)] {
                        return false;
                    }
                }
            }
            true
        }
    }
}

impl<T: PartialEq, const ROW: usize, const COL: usize> PartialEq<Matrix<T, ROW, COL>>
    for MatSlice<'_, T>
{
    fn eq(&self, other: &Matrix<T, ROW, COL>) -> bool {
        self.eq(&MatSlice::from(other))
    }
}

#[cfg(feature = "alloc")]
impl<T: PartialEq> PartialEq<DynMatrix<T>> for MatSlice<'_, T> {
    fn eq(&self, other: &DynMatrix<T>) -> bool {
        self.eq(&MatSlice::from(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_slice() {
        let mat = Matrix::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

        let slice = mat.slice(.., ..);
        assert_eq!(slice, mat);

        let sub1 = slice.slice(.., 1..);
        assert_eq!(sub1.rows(), 3);
        assert_eq!(sub1.cols(), 2);
        assert_eq!(sub1, Matrix::new([[2, 3], [5, 6], [8, 9]]));

        let sub2 = slice.slice(1.., ..);
        assert_eq!(sub2.rows(), 2);
        assert_eq!(sub2.cols(), 3);
        assert_eq!(sub2, Matrix::new([[4, 5, 6], [7, 8, 9]]));

        assert_eq!(sub1.slice(1.., ..), sub2.slice(.., 1..));
    }

    #[test]
    fn test_row_iter() {
        let mat = Matrix::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

        let slice = mat.slice(.., ..);

        let mut idx = 1;
        for row in slice.row_iter() {
            for item in row {
                assert_eq!(*item, idx);
                idx += 1;
            }
        }

        let sub1 = slice.slice(.., 1..);
        let mut idx = 2;
        for row in sub1.row_iter() {
            for item in row {
                assert_eq!(*item, idx);
                idx += 1;
            }
            idx += 1;
        }
    }

    #[test]
    fn test_column_iter() {
        let mat = Matrix::new([[1, 4, 7], [2, 5, 8], [3, 6, 9]]);

        let slice = mat.slice(.., ..);

        let mut idx = 1;
        for col in slice.column_iter() {
            for item in col {
                assert_eq!(*item, idx);
                idx += 1;
            }
        }

        let sub1 = slice.slice(1.., ..);
        let mut idx = 2;
        for col in sub1.column_iter() {
            for item in col {
                assert_eq!(*item, idx);
                idx += 1;
            }
            idx += 1;
        }
    }

    #[test]
    fn test_row_major_iter() {
        let mat = Matrix::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

        let slice = mat.slice(.., ..);

        let mut idx = 1;
        for item in slice.iter_row_major() {
            assert_eq!(*item, idx);
            idx += 1;
        }
    }

    #[test]
    fn test_column_major_iter() {
        let mat = Matrix::new([[1, 4, 7], [2, 5, 8], [3, 6, 9]]);

        let slice = mat.slice(.., ..);

        let mut idx = 1;
        for item in slice.iter_column_major() {
            assert_eq!(*item, idx);
            idx += 1;
        }
    }
}
