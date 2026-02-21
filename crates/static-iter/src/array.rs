//! Static length iterators over arrays

use super::{IntoStaticIter, StaticIter};
use core::mem::MaybeUninit;
use core::ptr;

impl<T, const N: usize> IntoStaticIter<N> for [T; N] {
    type Item = T;
    type Iter = IntoIter<T, N>;

    #[inline]
    fn into_static_iter(self) -> Self::Iter {
        IntoIter::new(self)
    }
}

impl<'a, T, const N: usize> IntoStaticIter<N> for &'a [T; N] {
    type Item = &'a T;
    type Iter = RefIter<'a, T, N>;

    #[inline]
    fn into_static_iter(self) -> Self::Iter {
        RefIter::new(self)
    }
}

impl<'a, T, const N: usize> IntoStaticIter<N> for &'a mut [T; N] {
    type Item = &'a mut T;
    type Iter = MutIter<'a, T, N>;

    #[inline]
    fn into_static_iter(self) -> Self::Iter {
        MutIter::new(self)
    }
}

/// Static iterator over owned values of an array
pub struct IntoIter<T, const N: usize> {
    inner: [MaybeUninit<T>; N],
}

impl<T, const N: usize> IntoIter<T, N> {
    #[inline]
    fn new(arr: [T; N]) -> Self {
        // SAFETY: `[MaybeUninit<T>; N]` and `[T; N]` have the same layout
        let inner = unsafe { core::mem::transmute_copy(&arr) };
        core::mem::forget(arr);
        IntoIter { inner }
    }
}

impl<T, const N: usize> StaticIter<N> for IntoIter<T, N> {
    type Item = T;

    #[inline]
    unsafe fn idx(&mut self, idx: usize) -> Self::Item {
        core::mem::replace(self.inner.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init()
    }
}

/// Static iterator over borrowed values of an array
pub struct RefIter<'a, T, const N: usize> {
    inner: &'a [T; N],
}

impl<'a, T, const N: usize> RefIter<'a, T, N> {
    #[inline]
    fn new(inner: &'a [T; N]) -> RefIter<'a, T, N> {
        RefIter { inner }
    }
}

impl<'a, T, const N: usize> StaticIter<N> for RefIter<'a, T, N> {
    type Item = &'a T;

    #[inline]
    unsafe fn idx(&mut self, idx: usize) -> Self::Item {
        &self.inner[idx]
    }
}

/// Static iterator over mutably borrowed values of an array
pub struct MutIter<'a, T, const N: usize> {
    inner: &'a mut [T; N],
}

impl<'a, T, const N: usize> MutIter<'a, T, N> {
    #[inline]
    fn new(inner: &'a mut [T; N]) -> MutIter<'a, T, N> {
        MutIter { inner }
    }
}

impl<'a, T, const N: usize> StaticIter<N> for MutIter<'a, T, N> {
    type Item = &'a mut T;

    #[inline]
    unsafe fn idx(&mut self, idx: usize) -> Self::Item {
        // SAFETY: Since this function can only be called once per index, each created reference
        //         points to a unique location, and can't outlive the underlying `'a` borrow.
        &mut *ptr::from_mut(&mut self.inner[idx]).cast::<Self::Item>()
    }
}
