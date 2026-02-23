//! Static length iterators over arrays

use super::{FromStaticIter, IntoStaticIter, StaticIter};
use core::convert::Infallible;
use core::mem::MaybeUninit;
use core::ops::ControlFlow;
use core::{mem, ptr};

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

impl<T, const N: usize> FromStaticIter<T, N> for [T; N] {
    type Uninit = [MaybeUninit<T>; N];
    type Break = Infallible;

    fn uninit() -> Self::Uninit {
        [const { MaybeUninit::uninit() }; N]
    }

    fn write(mut this: Self::Uninit, idx: usize, val: T) -> ControlFlow<Self::Break, Self::Uninit> {
        this[idx].write(val);
        ControlFlow::Continue(this)
    }

    unsafe fn finish(this: ControlFlow<Self::Break, Self::Uninit>) -> Self {
        let ControlFlow::Continue(c) = this;
        // SAFETY: `[T; N]` and `[MaybeUninit<T>; N]` have the same layout
        //         caller requirement that all values are initialized
        unsafe { mem::transmute_copy(&c) }
    }

    fn from_static_iter(mut iter: impl StaticIter<N, Item = T>) -> Self {
        // SAFETY: `from_fn` closure is guaranteed to be called exactly once for each index 0..N
        core::array::from_fn(|idx| unsafe { iter.idx(idx) })
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
        &mut *ptr::from_mut(&mut self.inner[idx]).cast::<T>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_iter() {
        let iter = [1, 2, 3, 4].into_static_iter();
        assert_eq!(iter.fold(0, |acc, val| acc + val), 10);
    }

    #[test]
    fn test_ref_iter() {
        let iter = (&[1, 2, 3, 4]).into_static_iter();
        assert_eq!(iter.fold(0, |acc, val| acc + *val), 10);
    }

    #[test]
    fn test_mut_iter() {
        let mut arr = [1, 2, 3, 4];
        let iter = (&mut arr).into_static_iter();
        iter.for_each(|val| *val += 1);
        assert_eq!(arr, [2, 3, 4, 5])
    }

    #[test]
    fn test_collect() {
        let arr = [1, 2, 3, 4].into_static_iter().collect::<[_; _]>();

        assert_eq!(arr, [1, 2, 3, 4])
    }
}
