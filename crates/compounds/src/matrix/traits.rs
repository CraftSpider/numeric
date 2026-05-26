use crate::matrix::{bound_end, bound_start, slice, MatMut, MatSlice};
use crate::vector::Vec2;
use core::ops::{IndexMut, RangeBounds};
use numeric_traits::class::RealSigned;

mod sealed {
    pub trait Sealed {}
}

pub(crate) use sealed::Sealed;

/// Matrix types - [`Matrix`](super::Matrix), [`DynMatrix`](super::DynMatrix), etc.
///
/// This doesn't include [`MatSlice`](MatSlice), which is a view into only part of a matrix.
pub trait MatrixLike:
    IndexMut<Vec2<usize>, Output = Self::Element>
    + IndexMut<(usize, usize), Output = Self::Element>
    + Sealed
{
    /// Element type of this matrix
    type Element;

    // Basic properties

    /// Get the number of rows in the matrix.
    fn rows(&self) -> usize;
    /// Get the number of columns in the matrix.
    fn cols(&self) -> usize;

    /// Get a pointer to the first element of this array. All other elements will be contiguous from
    /// this pointer, in row-major order.
    fn as_ptr(&self) -> *const Self::Element;

    /// Get a mutable pointer to the first element of this array. All other elements will be
    /// contiguous from this pointer, in row-major order.
    fn as_ptr_mut(&mut self) -> *mut Self::Element;

    // Slices

    /// Slice this array by rows and columns.
    ///
    /// The returned slice can only access elements within the specified bounds. To get a mutable
    /// slice, use [`Self::slice_mut`]
    fn slice(
        &self,
        rows: impl RangeBounds<usize>,
        cols: impl RangeBounds<usize>,
    ) -> MatSlice<'_, Self::Element> {
        let row_start = bound_start(rows.start_bound());
        let row_end = bound_end(rows.end_bound()).unwrap_or(self.rows());

        let col_start = bound_start(cols.start_bound());
        let col_end = bound_end(cols.end_bound()).unwrap_or(self.cols());

        let cols = self.cols();

        let ptr = unsafe { self.as_ptr().add(col_start).add(row_start * cols) };
        unsafe { MatSlice::from_raw_parts(ptr, row_end - row_start, col_end - col_start, cols) }
    }

    /// Mutably slice this array by rows and columns.
    ///
    /// The returned slice can only access elements within the specified bounds. To get an immutable
    /// slice, use [`Self::slice`]
    fn slice_mut(
        &mut self,
        rows: impl RangeBounds<usize>,
        cols: impl RangeBounds<usize>,
    ) -> MatMut<'_, Self::Element> {
        let row_start = bound_start(rows.start_bound());
        let row_end = bound_end(rows.end_bound()).unwrap_or(self.rows());

        let col_start = bound_start(cols.start_bound());
        let col_end = bound_end(cols.end_bound()).unwrap_or(self.cols());

        let cols = self.cols();

        let ptr = unsafe { self.as_ptr_mut().add(col_start).add(row_start * cols) };
        unsafe { MatMut::from_raw_parts(ptr, row_end - row_start, col_end - col_start, cols) }
    }

    // TODO: Split at row or column. get_many equivalent. Iterators.

    // Iterators

    fn row_iter(&self) -> slice::RowIter<'_, Self::Element> {
        todo!()
    }

    fn column_iter(&self) -> slice::ColumnIter<'_, Self::Element> {
        todo!()
    }

    fn iter_row_major(&self) -> slice::RowMajorIter<'_, Self::Element> {
        todo!()
    }

    fn iter_column_major(&self) -> slice::ColumnMajorIter<'_, Self::Element> {
        todo!()
    }

    // In-place mutations

    /// Swap two rows of the matrix
    fn swap_rows(&mut self, a: usize, b: usize) {
        self.slice_mut(.., ..).swap_rows(a, b);
    }

    /// Swap two columns of the matrix
    fn swap_columns(&mut self, a: usize, b: usize) {
        self.slice_mut(.., ..).swap_columns(a, b);
    }

    /// Uses Gaussian Elimination to put the matrix in triangular row-echelon form,
    /// and returns the new matrix and the product of the scalars by which the determinant was
    /// multiplied to produce the result.
    // TODO: This should be a signed integer op too, probably?
    fn gauss_elim(&mut self) -> Self::Element
    where
        Self::Element: RealSigned,
    {
        self.slice_mut(.., ..).gauss_elim()
    }
}
