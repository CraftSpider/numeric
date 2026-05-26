use crate::matrix::slice::size::PackedSize;
#[cfg(feature = "alloc")]
use crate::matrix::DynMatrix;
use crate::matrix::{index_check, Matrix, MatrixLike};
use crate::vector::Vec2;
use core::fmt;
use core::marker::PhantomData;
use core::ops::Index;
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
        stride: usize,
        cols: usize,
        rows: usize,
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
        unsafe { MatSlice::from_raw_parts(value.as_ptr(), COL, ROW, COL) }
    }
}

#[cfg(feature = "alloc")]
impl<'a, T> From<&'a DynMatrix<T>> for MatSlice<'a, T> {
    fn from(value: &'a DynMatrix<T>) -> Self {
        unsafe {
            MatSlice::from_raw_parts(value.as_ptr(), value.cols(), value.rows(), value.cols())
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
