use crate::matrix::slice::size::PackedSize;
#[cfg(feature = "alloc")]
use crate::matrix::DynMatrix;
use crate::matrix::{bound_end, bound_start, index_check, MatSlice, Matrix, MatrixLike};
use crate::vector::Vec2;
use core::marker::PhantomData;
use core::ops::{Deref, Index, IndexMut, RangeBounds};
use core::ptr;
use core::ptr::NonNull;
use numeric_traits::class::RealSigned;
use numeric_utils::static_assert;

static_assert!(size_of::<MatMut<'_, f32>>() == size_of::<Option<MatMut<'_, f32>>>());

/// Matrix sub-slice. A mutable view into a matrix, the same way a normal `[T]` slice is a view into
/// a contiguous array.
///
/// Mutable variant of [`MatSlice`].
#[repr(C)]
#[derive(Hash)]
pub struct MatMut<'a, T> {
    data: NonNull<T>,
    size: PackedSize,
    _phantom: PhantomData<&'a mut [T]>,
}

impl<'a, T> MatMut<'a, T> {
    /// Construct a mutable matrix slice from raw parts
    pub unsafe fn from_raw_parts(
        data: *mut T,
        rows: usize,
        cols: usize,
        stride: usize,
    ) -> MatMut<'a, T> {
        MatMut {
            data: NonNull::new_unchecked(data),
            size: PackedSize::new(rows, cols, stride),
            _phantom: PhantomData,
        }
    }

    /// Convert this mutable matrix slice into raw parts
    pub fn into_raw_parts(self) -> (NonNull<T>, usize, usize, usize) {
        let (rows, cols, stride) = self.size.unpack();
        (self.data, rows, cols, stride)
    }

    /// Get a sub-slice of this matrix slice, by rows and columns.
    pub fn slice_mut(
        &self,
        rows: impl RangeBounds<usize>,
        cols: impl RangeBounds<usize>,
    ) -> MatMut<'_, T> {
        let stride = self.size.stride();

        let row_start = bound_start(rows.start_bound());
        let row_end = bound_end(rows.end_bound()).unwrap_or(self.rows());

        let col_start = bound_start(cols.start_bound());
        let col_end = bound_end(cols.end_bound()).unwrap_or(self.cols());

        let ptr = unsafe { self.data.add(col_start).add(row_start * stride) };
        unsafe {
            MatMut::from_raw_parts(
                ptr.as_ptr(),
                row_end - row_start,
                col_end - col_start,
                stride,
            )
        }
    }

    /// Swap two rows of the matrix
    pub fn swap_rows(&mut self, a: usize, b: usize) {
        let (_, cols, stride) = self.size.unpack();
        let a = unsafe { self.data.add(a * stride) };
        let b = unsafe { self.data.add(b * stride) };
        unsafe { ptr::swap_nonoverlapping(a.as_ptr(), b.as_ptr(), cols) };
    }

    /// Swap two columns of the matrix
    pub fn swap_columns(&mut self, a: usize, b: usize) {
        let (rows, _, stride) = self.size.unpack();
        for i in 0..rows {
            let a = unsafe { self.data.add(i * stride + a) };
            let b = unsafe { self.data.add(i * stride + b) };
            unsafe { ptr::swap_nonoverlapping(a.as_ptr(), b.as_ptr(), 1) };
        }
    }

    /// Uses Gaussian Elimination to put the matrix in triangular row-echelon form,
    /// and returns the new matrix and the product of the scalars by which the determinant was
    /// multiplied to produce the result.
    // TODO: This should be a signed integer op too, probably?
    pub fn gauss_elim(&mut self) -> T
    where
        T: RealSigned,
    {
        let mut factor = T::one();
        let mut row = 0;
        for col in 0..self.cols() {
            // We always act on submatrix [row..][col..]

            // If all columns are 0, skip
            let all_zero = (row..self.rows()).all(|r| self[(r, col)].is_zero());
            if all_zero {
                continue;
            }

            // Otherwise, find smallest non-zero number, move to top
            let (small_idx, _) = (row..self.rows())
                .map(|r| &self[(r, col)])
                .enumerate()
                .filter(|(_, val)| !val.is_zero())
                .min_by(|(_, val1), (_, val2)| val1.partial_cmp(val2).unwrap())
                .unwrap();
            let small_idx = small_idx + row;

            if small_idx != row {
                factor = factor.neg();

                self.swap_rows(small_idx, row);
            }

            // Remove all other values in this column by subtracting top row
            //   if it doesn't go evenly into some values, multiply by factor to make it so
            let top = self[(row, col)].clone();
            for r in row + 1..self.rows() {
                let factor = self[(r, col)].clone() / top.clone();
                for c in col..self.cols() {
                    self[(r, c)] = self[(r, c)].clone() - self[(row, c)].clone() * factor.clone();
                }
            }

            // Increment row we're operating on
            row += 1;
        }

        factor
    }
}

impl<'a, T> Deref for MatMut<'a, T> {
    type Target = MatSlice<'a, T>;

    fn deref(&self) -> &Self::Target {
        let ptr = ptr::from_ref(self).cast();
        unsafe { &*ptr }
    }
}

impl<'a, T> Index<(usize, usize)> for MatMut<'a, T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        (**self).index(index)
    }
}

impl<'a, T> Index<Vec2<usize>> for MatMut<'a, T> {
    type Output = T;

    fn index(&self, index: Vec2<usize>) -> &Self::Output {
        &self[(*index.x(), *index.y())]
    }
}

impl<'a, T> IndexMut<(usize, usize)> for MatMut<'a, T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (rows, cols, stride) = self.size.unpack();
        index_check((rows, cols), index);
        // SAFETY: Internal pointer guaranteed valid for mutable access in checked range
        unsafe { self.data.add(index.0 * stride + index.1).as_mut() }
    }
}

impl<'a, T> IndexMut<Vec2<usize>> for MatMut<'a, T> {
    fn index_mut(&mut self, index: Vec2<usize>) -> &mut Self::Output {
        &mut self[(*index.x(), *index.y())]
    }
}

impl<'a, T, const ROW: usize, const COL: usize> From<&'a mut Matrix<T, ROW, COL>>
    for MatMut<'a, T>
{
    fn from(value: &'a mut Matrix<T, ROW, COL>) -> Self {
        unsafe { MatMut::from_raw_parts(value.as_ptr_mut(), ROW, COL, COL) }
    }
}

#[cfg(feature = "alloc")]
impl<'a, T> From<&'a mut DynMatrix<T>> for MatMut<'a, T> {
    fn from(value: &'a mut DynMatrix<T>) -> Self {
        unsafe {
            MatMut::from_raw_parts(value.as_ptr_mut(), value.rows(), value.cols(), value.cols())
        }
    }
}
