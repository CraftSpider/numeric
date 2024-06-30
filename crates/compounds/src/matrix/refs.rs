use crate::matrix::{DynMatrix, Matrix};
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
                unsafe { &*self.data.as_ptr().add(index.0 * self.cols + index.1) }
            }
        }
    };
}

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

#[cfg(feature = "std")]
impl<'a, T> From<&'a DynMatrix<T>> for MatrixRef<'a, T> {
    fn from(value: &'a DynMatrix<T>) -> Self {
        MatrixRef::new(value.as_ptr(), value.rows(), value.cols())
    }
}

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
        if index.0 > self.rows || index.1 > self.cols {
            panic!(
                "Index out of range for matrix of size {}x{}: ({},{})",
                self.rows, self.cols, index.0, index.1
            );
        }

        unsafe { &mut *self.data.as_ptr().add(index.0 * self.cols + index.1) }
    }
}

impl<'a, T, const ROW: usize, const COL: usize> From<&'a mut Matrix<T, ROW, COL>>
    for MatrixMut<'a, T>
{
    fn from(value: &'a mut Matrix<T, ROW, COL>) -> Self {
        MatrixMut::new(value.as_mut_ptr(), ROW, COL)
    }
}

#[cfg(feature = "std")]
impl<'a, T> From<&'a mut DynMatrix<T>> for MatrixMut<'a, T> {
    fn from(value: &'a mut DynMatrix<T>) -> Self {
        MatrixMut::new(value.as_mut_ptr(), value.rows(), value.cols())
    }
}
