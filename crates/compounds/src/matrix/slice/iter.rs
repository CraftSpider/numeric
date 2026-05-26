use core::ptr::NonNull;
use core::{iter, slice};

pub struct RowItemIter<'a, T>(slice::Iter<'a, T>);

impl<'a, T: 'a> Iterator for RowItemIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub struct RowIter<'a, T> {
    _phantom: core::marker::PhantomData<&'a T>,
}

impl<'a, T: 'a> Iterator for RowIter<'a, T> {
    type Item = RowItemIter<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub struct ColumnItemIter<'a, T> {
    ptr: NonNull<T>,
    end: *const T,
    stride: usize,
    _phantom: core::marker::PhantomData<&'a T>,
}

impl<'a, T: 'a> Iterator for ColumnItemIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr.as_ptr().cast_const() == self.end {
            return None;
        }
        let out = unsafe { self.ptr.as_ref() };
        self.ptr = unsafe { self.ptr.add(self.stride) };
        Some(out)
    }
}

pub struct ColumnIter<'a, T> {
    _phantom: core::marker::PhantomData<&'a T>,
}

impl<'a, T: 'a> Iterator for ColumnIter<'a, T> {
    type Item = ColumnItemIter<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub struct RowMajorIter<'a, T>(iter::Flatten<RowIter<'a, T>>);

impl<'a, T: 'a> Iterator for RowMajorIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub struct ColumnMajorIter<'a, T>(iter::Flatten<ColumnIter<'a, T>>);

impl<'a, T: 'a> Iterator for ColumnMajorIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
