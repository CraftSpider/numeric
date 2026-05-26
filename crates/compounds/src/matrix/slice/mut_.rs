use crate::matrix::slice::size::PackedSize;
#[cfg(feature = "alloc")]
use crate::matrix::DynMatrix;
use crate::matrix::{
    bound_end, bound_start, index_check, range_check, MatSlice, Matrix, MatrixLike,
};
use crate::vector::Vec2;
use core::marker::PhantomData;
use core::ops::{Index, IndexMut, RangeBounds};
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
    pub(crate) fn new_raw(data: NonNull<T>, size: PackedSize) -> MatMut<'a, T> {
        MatMut {
            data,
            size,
            _phantom: PhantomData,
        }
    }

    /// Construct a mutable matrix slice from raw parts
    ///
    /// # Safety
    ///
    /// The provided pointer must point to a valid position in a matrix in row-major order.
    ///
    /// It must be safe to write contiguous ranges of length `cols` every `stride` elements,
    /// `rows` times. Items in between these ranges need not be valid for reads.
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

    /// Get the number of rows in the matrix
    pub fn rows(&self) -> usize {
        self.size.len()
    }

    /// Get the number of columns in the matrix
    pub fn cols(&self) -> usize {
        self.size.depth()
    }

    pub(crate) fn size(&self) -> PackedSize {
        self.size
    }

    pub(crate) fn ptr(&self) -> NonNull<T> {
        self.data
    }

    /// Reborrow as an immutable slice for a shorter lifetime
    pub fn immut(&self) -> MatSlice<'_, T> {
        MatSlice::new_raw(self.data, self.size)
    }

    /// Reborrow for a shorter lifetime
    pub fn reborrow(&mut self) -> MatMut<'_, T> {
        MatMut::new_raw(self.data, self.size)
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
            MatMut::from_raw_parts(
                ptr.as_ptr(),
                row_end - row_start,
                col_end - col_start,
                stride,
            )
        }
    }

    /// Swap two rows of the matrix
    ///
    /// # Panics
    ///
    /// If either `a` or `b` are out-of-bounds of this slice.
    pub fn swap_rows(&mut self, a: usize, b: usize) {
        let (rows, cols, stride) = self.size.unpack();

        if a >= rows {
            panic!(
                "Row out of range for matrix of size {}x{}: {}",
                rows, cols, a
            );
        } else if b >= rows {
            panic!(
                "Row out of range for matrix of size {}x{}: {}",
                rows, cols, b
            );
        }

        if a == b {
            return;
        }

        // SAFETY: `a` guaranteed in-bounds
        let a = unsafe { self.data.add(a * stride) };
        // SAFETY: `b` guaranteed in-bounds
        let b = unsafe { self.data.add(b * stride) };
        // SAFETY: `a` and `b` point to starts of separate rows
        unsafe { ptr::swap_nonoverlapping(a.as_ptr(), b.as_ptr(), cols) };
    }

    /// Swap two columns of the matrix
    ///
    /// # Panics
    ///
    /// If either `a` or `b` are out-of-bounds of this slice.
    pub fn swap_columns(&mut self, a: usize, b: usize) {
        let (rows, cols, stride) = self.size.unpack();

        if a >= cols {
            panic!(
                "Column out of range for matrix of size {}x{}: {}",
                rows, cols, a
            );
        } else if b >= cols {
            panic!(
                "Column out of range for matrix of size {}x{}: {}",
                rows, cols, b
            );
        }

        if a == b {
            return;
        }

        for i in 0..rows {
            // SAFETY: `a` guaranteed in-bounds
            let a = unsafe { self.data.add(i * stride + a) };
            // SAFETY: `b` guaranteed in-bounds
            let b = unsafe { self.data.add(i * stride + b) };
            // SAFETY: `a` and `b` point to elements of separate columns
            unsafe { ptr::swap_nonoverlapping(a.as_ptr(), b.as_ptr(), 1) };
        }
    }

    /// Uses Gaussian Elimination to put the matrix in triangular row-echelon form,
    /// and returns the new matrix and the product of the scalars by which the determinant was
    /// multiplied to produce the result.
    // TODO: This should be a signed integer op too, probably?
    #[allow(clippy::missing_panics_doc)]
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

// TODO: This is unsound :(
// impl<'a, T> Deref for MatMut<'a, T> {
//     type Target = MatSlice<'a, T>;
//
//     fn deref(&self) -> &Self::Target {
//         let ptr = ptr::from_ref(self).cast();
//         // SAFETY: `MatMut` is guaranteed to have the same layout as `MatSlice` due to both being
//         //         `repr(C)` and having the same elements.
//         unsafe { &*ptr }
//     }
// }

impl<'a, T> Index<(usize, usize)> for MatMut<'a, T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (rows, cols, stride) = self.size.unpack();
        index_check((rows, cols), index);
        // SAFETY: Internal pointer guaranteed valid for immutable access in checked range
        unsafe { self.data.add(index.0 * stride + index.1).as_ref() }
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
        // SAFETY: Arguments trivially valid by definition
        unsafe { MatMut::from_raw_parts(value.as_ptr_mut(), ROW, COL, COL) }
    }
}

#[cfg(feature = "alloc")]
impl<'a, T> From<&'a mut DynMatrix<T>> for MatMut<'a, T> {
    fn from(value: &'a mut DynMatrix<T>) -> Self {
        // SAFETY: Arguments trivially valid by definition
        unsafe {
            MatMut::from_raw_parts(value.as_ptr_mut(), value.rows(), value.cols(), value.cols())
        }
    }
}
