#[cfg(feature = "alloc")]
use crate::matrix::DynMatrix;
use crate::matrix::Matrix;
use core::marker::PhantomData;
use core::ops::{Index, IndexMut};
use core::ptr::NonNull;

macro_rules! ref_common {
    ($ty:ty) => {
        impl<'a, T> Index<(usize, usize)> for $ty {
            type Output = T;

            fn index(&self, index: (usize, usize)) -> &Self::Output {
                if index.0 > self.rows || index.1 > self.cols {
                    panic!(
                        "Index out of range for matrix of size {}x{}: ({},{})",
                        self.rows, self.cols, index.0, index.1
                    );
                }

                // SAFETY: Internal pointer guaranteed valid for reads up to rows * cols
                unsafe { self.data.add(index.0 * self.cols + index.1).as_ref() }
            }
        }
    };
}

/// Immutable reference view into a matrix.
pub struct MatrixRef<'a, T> {
    data: NonNull<T>,
    rows: usize,
    cols: usize,
    _phantom: PhantomData<&'a [T]>,
}

impl<'a, T> MatrixRef<'a, T> {
    fn new(data: NonNull<T>, rows: usize, cols: usize) -> MatrixRef<'a, T> {
        MatrixRef {
            data,
            rows,
            cols,
            _phantom: PhantomData,
        }
    }
}

ref_common!(MatrixRef<'a, T>);

impl<'a, T> Copy for MatrixRef<'a, T> {}

impl<'a, T> Clone for MatrixRef<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T, const ROW: usize, const COL: usize> From<&'a Matrix<T, ROW, COL>> for MatrixRef<'a, T> {
    fn from(value: &'a Matrix<T, ROW, COL>) -> Self {
        MatrixRef::new(value.as_ptr(), ROW, COL)
    }
}

#[cfg(feature = "alloc")]
impl<'a, T> From<&'a DynMatrix<T>> for MatrixRef<'a, T> {
    fn from(value: &'a DynMatrix<T>) -> Self {
        MatrixRef::new(value.as_ptr(), value.rows(), value.cols())
    }
}

/// Mutable reference view into a matrix.
pub struct MatrixMut<'a, T> {
    data: NonNull<T>,
    rows: usize,
    cols: usize,
    _phantom: PhantomData<&'a mut [T]>,
}

impl<'a, T> MatrixMut<'a, T> {
    fn new(data: NonNull<T>, rows: usize, cols: usize) -> MatrixMut<'a, T> {
        MatrixMut {
            data,
            rows,
            cols,
            _phantom: PhantomData,
        }
    }
}

ref_common!(MatrixMut<'a, T>);

impl<T> IndexMut<(usize, usize)> for MatrixMut<'_, T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        assert!(
            index.0 < self.rows && index.1 < self.cols,
            "Index out of range for matrix of size {}x{}: ({},{})",
            self.rows,
            self.cols,
            index.0,
            index.1
        );
        // SAFETY: Internal pointer guaranteed valid for reads and writes up to rows * cols
        unsafe { self.data.add(index.0 * self.cols + index.1).as_mut() }
    }
}

impl<'a, T, const ROW: usize, const COL: usize> From<&'a mut Matrix<T, ROW, COL>>
    for MatrixMut<'a, T>
{
    fn from(value: &'a mut Matrix<T, ROW, COL>) -> Self {
        MatrixMut::new(value.as_mut_ptr(), ROW, COL)
    }
}

#[cfg(feature = "alloc")]
impl<'a, T> From<&'a mut DynMatrix<T>> for MatrixMut<'a, T> {
    fn from(value: &'a mut DynMatrix<T>) -> Self {
        MatrixMut::new(value.as_mut_ptr(), value.rows(), value.cols())
    }
}

// Pointer is to first accessible column
// stride is total columns, distance to jump to move one row
// cols is accessible columns, how far forward we can move from ptr
// rows is accessible rows, how many times we can add stride
pub struct MatrixSlice<'a, T> {
    data: NonNull<T>,
    // >= cols <= isize::MAX
    stride: usize,
    // <= isize::MAX
    cols: usize,
    // <= isize::MAX
    rows: usize,
    _phantom: PhantomData<&'a [T]>,
}

impl<'a, T> MatrixSlice<'a, T> {
    fn new(data: NonNull<T>, stride: usize, cols: usize, rows: usize) -> MatrixSlice<'a, T> {
        MatrixSlice {
            data,
            stride,
            cols,
            rows,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T> Index<(usize, usize)> for MatrixSlice<'a, T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        if index.0 > self.rows || index.1 > self.cols {
            panic!(
                "Index out of range for matrix of size {}x{}: ({},{})",
                self.rows, self.cols, index.0, index.1
            );
        }

        // SAFETY: Internal pointer guaranteed valid for reads in checked range
        unsafe { self.data.add(index.0 * self.stride + index.1).as_ref() }
    }
}

impl<'a, T, const ROW: usize, const COL: usize> From<&'a mut Matrix<T, ROW, COL>>
    for MatrixSlice<'a, T>
{
    fn from(value: &'a mut Matrix<T, ROW, COL>) -> Self {
        MatrixSlice::new(value.as_mut_ptr(), COL, ROW, COL)
    }
}

impl<'a, T> From<&'a DynMatrix<T>> for MatrixSlice<'a, T> {
    fn from(value: &'a DynMatrix<T>) -> Self {
        MatrixSlice::new(value.as_ptr(), value.cols(), value.rows(), value.cols())
    }
}

#[cfg(test)]
mod tests {}
