use crate::matrix::slice::size::PackedSize;
#[cfg(feature = "alloc")]
use crate::matrix::DynMatrix;
use crate::matrix::{bound_end, bound_start, index_check, Matrix, MatrixLike};
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
    /// Construct a matrix slice from raw parts
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

    /// Get a sub-slice of this matrix slice, by rows and columns.
    pub fn slice(
        &self,
        rows: impl RangeBounds<usize>,
        cols: impl RangeBounds<usize>,
    ) -> MatSlice<'_, T> {
        let stride = self.size.stride();

        let row_start = bound_start(rows.start_bound());
        let row_end = bound_end(rows.end_bound()).unwrap_or(self.rows());

        let col_start = bound_start(cols.start_bound());
        let col_end = bound_end(cols.end_bound()).unwrap_or(self.cols());

        let ptr = unsafe { self.data.add(col_start).add(row_start * stride) };
        unsafe {
            MatSlice::from_raw_parts(
                ptr.as_ptr(),
                row_end - row_start,
                col_end - col_start,
                stride,
            )
        }
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

impl<'a, T, const ROW: usize, const COL: usize> From<&'a Matrix<T, ROW, COL>> for MatSlice<'a, T> {
    fn from(value: &'a Matrix<T, ROW, COL>) -> Self {
        unsafe { MatSlice::from_raw_parts(value.as_ptr(), ROW, COL, COL) }
    }
}

#[cfg(feature = "alloc")]
impl<'a, T> From<&'a DynMatrix<T>> for MatSlice<'a, T> {
    fn from(value: &'a DynMatrix<T>) -> Self {
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
}
