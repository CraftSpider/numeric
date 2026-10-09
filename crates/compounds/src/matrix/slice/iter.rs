use crate::matrix::{MatSlice, NonNullExt};
use core::ptr::NonNull;
use core::{iter, slice};

/// Iterator over the items in a row of a matrix
pub struct RowItemIter<'a, T>(slice::Iter<'a, T>);

impl<'a, T> RowItemIter<'a, T> {
    fn new(ptr: NonNull<T>, cols: usize) -> Self {
        // SAFETY: As matrix slices are row-major, the pointer is guaranteed to be valid for reads
        //         for `cols` elements.
        RowItemIter(unsafe { slice::from_raw_parts(ptr.as_ptr(), cols) }.iter())
    }
}

impl<'a, T: 'a> Iterator for RowItemIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

/// Iterator over the rows of a matrix
pub struct RowIter<'a, T> {
    inner: MatSlice<'a, T>,
}

impl<'a, T> RowIter<'a, T> {
    pub(crate) fn new(inner: MatSlice<'a, T>) -> Self {
        RowIter { inner }
    }
}

impl<'a, T: 'a> Iterator for RowIter<'a, T> {
    type Item = RowItemIter<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.rows() == 0 {
            return None;
        }
        let out = RowItemIter::new(self.inner.ptr(), self.inner.cols());
        self.inner = self.inner.slice(1.., ..);
        Some(out)
    }
}

/// Iterator over the items in a column of a matrix
pub struct ColumnItemIter<'a, T> {
    ptr: NonNull<T>,
    end: *const T,
    stride: usize,
    _phantom: core::marker::PhantomData<&'a T>,
}

impl<'a, T> ColumnItemIter<'a, T> {
    fn new(ptr: NonNull<T>, rows: usize, stride: usize) -> Self {
        let end = ptr.as_ptr().wrapping_add(rows * stride);
        ColumnItemIter {
            ptr,
            end,
            stride,
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<'a, T: 'a> Iterator for ColumnItemIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr.as_ptr().cast_const() == self.end {
            return None;
        }
        // SAFETY: Internal pointer guaranteed valid till we reach `end`
        let out = unsafe { self.ptr.as_ref() };
        // SAFETY: Pointer guaranteed to have provenance up till `end`
        self.ptr = self.ptr.wrapping_add(self.stride);
        Some(out)
    }
}

/// Iterator over the columns of a matrix
pub struct ColumnIter<'a, T> {
    inner: MatSlice<'a, T>,
}

impl<'a, T> ColumnIter<'a, T> {
    pub(crate) fn new(inner: MatSlice<'a, T>) -> Self {
        ColumnIter { inner }
    }
}

impl<'a, T: 'a> Iterator for ColumnIter<'a, T> {
    type Item = ColumnItemIter<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.cols() == 0 {
            return None;
        }
        let out = ColumnItemIter::new(self.inner.ptr(), self.inner.rows(), self.inner.stride());
        self.inner = self.inner.slice(.., 1..);
        Some(out)
    }
}

/// Iterator over the items in a matrix, in row-major order.
pub struct RowMajorIter<'a, T>(iter::Flatten<RowIter<'a, T>>);

impl<'a, T> RowMajorIter<'a, T> {
    pub(crate) fn new(inner: MatSlice<'a, T>) -> Self {
        RowMajorIter(RowIter::new(inner).flatten())
    }
}

impl<'a, T> Iterator for RowMajorIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

/// Iterator over the items in a matrix, in column-major order.
pub struct ColumnMajorIter<'a, T>(iter::Flatten<ColumnIter<'a, T>>);

impl<'a, T> ColumnMajorIter<'a, T> {
    pub(crate) fn new(inner: MatSlice<'a, T>) -> Self {
        ColumnMajorIter(ColumnIter::new(inner).flatten())
    }
}

impl<'a, T: 'a> Iterator for ColumnMajorIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
