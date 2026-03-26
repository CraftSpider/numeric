//! Implementation of a static-length variant of the [`Iterator`] trait. This often allows
//! for better codegen, and also is just generally more convenient when working with arrays or
//! homogenous tuples.

#![no_std]

use adapter::{Enumerate, Map, Zip};
use core::ops::{Add, ControlFlow, Mul};
use numeric_traits::identity::{One, Zero};

pub mod adapter;
pub mod array;
#[cfg(false)]
pub mod codegen;
pub mod tuple;
pub mod zip_all;

use crate::adapter::{Cloned, Copied, Rev, Take};
pub use zip_all::zip_all;

/// Types that can be collected from a static length iterator.
pub trait FromStaticIter<T, const N: usize>: Sized {
    /// The uninitialized state of the collection. This is used to build up the collection, and then
    /// call `finish` to convert it into a final collection.
    type Uninit;

    /// The type returned if a write fails, and iteration should stop early. For iterators where
    /// write can't fail except in bug cases, this will be [`Infallible`].
    type Break;

    /// Create a new uninitialized collection.
    fn uninit() -> Self::Uninit;

    /// Write a value into the collection at the given index. Out of bounds writes must be sound,
    /// but may panic or otherwise misbehave.
    fn write(this: Self::Uninit, idx: usize, val: T) -> ControlFlow<Self::Break, Self::Uninit>;

    /// # Safety
    ///
    /// This will be called with uninit data in two cases:
    /// 1) The underlying iterator has been polled to completion, and `write` called once for each
    ///    index
    /// 2) Write returned `ControlFlow::Break` at any point
    unsafe fn finish(this: ControlFlow<Self::Break, Self::Uninit>) -> Self;

    /// Helper to build this collection from a static length iterator, without having to manage the
    /// unsafe details of indexing and finishing.
    fn from_static_iter(mut iter: impl StaticIter<N, Item = T>) -> Self {
        let uninit = (0..N).try_fold(Self::uninit(), |acc, idx| {
            // SAFETY: idx is in 0..N, and takes each value at most once
            let val = unsafe { iter.idx(idx) };
            Self::write(acc, idx, val)
        });
        // SAFETY: We either reach the end and wrote all values from 0..N, or have a break value
        unsafe { Self::finish(uninit) }
    }
}

impl<T, C, const N: usize> FromStaticIter<Option<T>, N> for Option<C>
where
    C: FromStaticIter<T, N>,
{
    type Uninit = C::Uninit;
    type Break = Option<C::Break>;

    fn uninit() -> Self::Uninit {
        C::uninit()
    }

    fn write(
        this: Self::Uninit,
        idx: usize,
        val: Option<T>,
    ) -> ControlFlow<Self::Break, Self::Uninit> {
        match val {
            Some(val) => C::write(this, idx, val).map_break(Some),
            None => ControlFlow::Break(None),
        }
    }

    unsafe fn finish(this: ControlFlow<Self::Break, Self::Uninit>) -> Self {
        match this {
            ControlFlow::Continue(inner) => Some(C::finish(ControlFlow::Continue(inner))),
            ControlFlow::Break(Some(inner)) => Some(C::finish(ControlFlow::Break(inner))),
            ControlFlow::Break(None) => None,
        }
    }
}

impl<T, E, C, const N: usize> FromStaticIter<Result<T, E>, N> for Result<C, E>
where
    C: FromStaticIter<T, N>,
{
    type Uninit = C::Uninit;
    type Break = Result<C::Break, E>;

    fn uninit() -> Self::Uninit {
        C::uninit()
    }

    fn write(
        this: Self::Uninit,
        idx: usize,
        val: Result<T, E>,
    ) -> ControlFlow<Self::Break, Self::Uninit> {
        match val {
            Ok(val) => C::write(this, idx, val).map_break(Ok),
            Err(e) => ControlFlow::Break(Err(e)),
        }
    }

    unsafe fn finish(this: ControlFlow<Self::Break, Self::Uninit>) -> Self {
        match this {
            ControlFlow::Continue(inner) => Ok(C::finish(ControlFlow::Continue(inner))),
            ControlFlow::Break(Ok(e)) => Ok(C::finish(ControlFlow::Break(e))),
            ControlFlow::Break(Err(e)) => Err(e),
        }
    }
}

/// Static length iterators. These are similar to [`Iterator`], but have a length known at compile-
/// time. This allows them to be collected directly into arrays, homogenous tuples, or other similar
/// types infallibly. It also sometimes allows for improved codegen of loops.
pub trait StaticIter<const N: usize>: Sized {
    /// The type of values returned by the iterator.
    type Item;

    /// # Safety
    ///
    /// This function must be called at most once for each index in order starting from zero
    /// The index must be in the range 0..N
    unsafe fn idx(&mut self, idx: usize) -> Self::Item;

    /// Takes a closure, and returns an iterator which calls that closure on each element.
    #[inline]
    fn map<T, F: FnMut(Self::Item) -> T>(self, func: F) -> Map<Self, F> {
        Map { inner: self, func }
    }

    /// 'Zips up' two iterators into a single iterator of pairs.
    #[inline]
    fn zip<I>(self, other: I) -> Zip<Self, I::Iter>
    where
        I: IntoStaticIter<N>,
    {
        Zip {
            left: self,
            right: other.into_static_iter(),
        }
    }

    /// Returns an iterator that will yield a tuple of (index, value) for each item in this
    /// iterator.
    #[inline]
    fn enumerate(self) -> Enumerate<Self> {
        Enumerate { inner: self }
    }

    /// Clone each value in the iterator, converting an iterator of references to an iterator of
    /// owned values
    #[inline]
    fn cloned<'a, T>(self) -> Cloned<Self>
    where
        T: Copy + 'a,
        Self: StaticIter<N, Item = &'a T>,
    {
        Cloned { inner: self }
    }

    /// Copy each value in the iterator, converting an iterator of references to an iterator of
    /// owned values
    #[inline]
    fn copied<'a, T>(self) -> Copied<Self>
    where
        T: Copy + 'a,
        Self: StaticIter<N, Item = &'a T>,
    {
        Copied { inner: self }
    }

    /// Reverse the iterator's direction
    #[inline]
    fn rev(self) -> Rev<Self> {
        Rev { inner: self }
    }

    /// Take the first M elements of this iterator
    #[inline]
    fn take<const M: usize>(self) -> Take<Self, N, M> {
        const { assert!(M <= N) };
        Take { inner: self }
    }

    /// Given a starting value and a closure, call the closure with either the starting value or
    /// the result of the previous call and the next iteration value for every item in the iterator.
    #[inline]
    fn fold<T, F: FnMut(T, Self::Item) -> T>(mut self, start: T, mut func: F) -> T {
        (0..N).fold(start, |acc, idx| {
            // SAFETY: Follows contract of `idx` - we call exactly once for each value from `0..N`
            let item = unsafe { self.idx(idx) };
            func(acc, item)
        })
    }

    // TODO: This really wants to use `Try`
    /// Given a starting value and a closure, call the closure with either the starting value or
    /// the result of the previous call and the next iteration value for every item in the iterator.
    ///
    /// If any call returns an error, stop iterating immediately and return that error.
    #[inline]
    fn try_fold<T, E, F: FnMut(T, Self::Item) -> Result<T, E>>(
        mut self,
        start: T,
        mut func: F,
    ) -> Result<T, E> {
        // TODO: This leaks un-called values
        (0..N).try_fold(start, |acc, idx| {
            // SAFETY: Follows contract of `idx` - we call exactly once for each value from `0..N`
            let item = unsafe { self.idx(idx) };
            func(acc, item)
        })
    }

    /// Given a closure, call it for each item in the iterator eagerly. This immediately consumes
    /// the iterator, and is equivalent to `for _ in _`.
    #[inline]
    fn for_each<F: FnMut(Self::Item)>(self, mut func: F) {
        self.fold((), |(), i| func(i))
    }

    /// Collect the values from this iterator into an output collection
    fn collect<C: FromStaticIter<Self::Item, N>>(self) -> C {
        C::from_static_iter(self)
    }

    /// Apply a closure to every item in this iterator, returning true if any call returns true.
    fn any<F: FnMut(Self::Item) -> bool>(self, mut func: F) -> bool {
        self.try_fold((), |(), x| if func(x) { Err(()) } else { Ok(()) }) == Err(())
    }

    /// Apply a closure to every item in this iterator, returning true if all calls return true.
    fn all<F: FnMut(Self::Item) -> bool>(self, mut func: F) -> bool {
        self.try_fold((), |(), x| if func(x) { Ok(()) } else { Err(()) }) == Ok(())
    }

    // TODO: Move this and sum to an extension in numeric-traits? Makes static_iter stand alone
    /// Sum the values in this iterator.
    fn sum(self) -> Self::Item
    where
        Self::Item: Zero + Add<Output = Self::Item>,
    {
        self.fold(Self::Item::zero(), |acc, val| acc + val)
    }

    /// Get the product of the values in this iterator.
    fn product(self) -> Self::Item
    where
        Self::Item: One + Mul<Output = Self::Item>,
    {
        self.fold(Self::Item::one(), |acc, val| acc * val)
    }
}

/// Types that can be converted into a static length iterator.
pub trait IntoStaticIter<const N: usize> {
    /// The type of values returned by the iterator.
    type Item;
    /// The type of the iterator returned by [`Self::into_static_iter`].
    type Iter: StaticIter<N, Item = Self::Item>;

    /// Convert this value into a static length iterator.
    fn into_static_iter(self) -> Self::Iter;
}

impl<I: StaticIter<N>, const N: usize> IntoStaticIter<N> for I {
    type Item = I::Item;
    type Iter = I;

    fn into_static_iter(self) -> Self::Iter {
        self
    }
}

/// Type similar to `..N`, but with compile-time value
pub struct StaticRangeTo<const N: usize>;

impl<const N: usize> IntoStaticIter<N> for StaticRangeTo<N> {
    type Item = usize;
    type Iter = StaticRangeToIter<N>;

    fn into_static_iter(self) -> Self::Iter {
        StaticRangeToIter::<N>(())
    }
}

/// Iterator type for [`StaticRangeTo`]
pub struct StaticRangeToIter<const N: usize>(());

impl<const N: usize> StaticIter<N> for StaticRangeToIter<N> {
    type Item = usize;

    unsafe fn idx(&mut self, idx: usize) -> Self::Item {
        idx
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::str::FromStr;

    #[test]
    fn test_zip_add() {
        let res: [i32; 4] = [1, 2, 3, 4]
            .into_static_iter()
            .zip([5, 6, 7, 8].into_static_iter())
            .map(|(l, r)| l + r)
            .collect();

        assert_eq!(res, [6, 8, 10, 12]);
    }

    #[test]
    fn test_option_collect() {
        let res: [u32; 4] = [1u32, 2, 3, 4]
            .into_static_iter()
            .map(|l| l.checked_add(1))
            .collect::<Option<_>>()
            .unwrap();
        assert_eq!(res, [2, 3, 4, 5]);

        let res: Option<[u32; 4]> = [u32::MAX - 1, u32::MAX, 0, 1]
            .into_static_iter()
            .map(|l| l.checked_add(1))
            .collect::<Option<_>>();
        assert!(res.is_none());
    }

    #[test]
    fn test_option_option_collect() {
        let res: [u32; 4] = [1u32, 2, 3, 4]
            .into_static_iter()
            .map(|l| Some(l.checked_add(1)))
            .collect::<Option<Option<_>>>()
            .unwrap()
            .unwrap();
        assert_eq!(res, [2, 3, 4, 5]);

        let res: Option<[u32; 4]> = [u32::MAX - 1, u32::MAX, 0, 1]
            .into_static_iter()
            .map(|l| Some(l.checked_add(1)))
            .collect::<Option<_>>()
            .unwrap();
        assert!(res.is_none());

        let res: Option<[u32; 4]> = [u32::MAX - 1, u32::MAX, 0, 1]
            .into_static_iter()
            .map(|_| None)
            .collect::<Option<_>>();
        assert!(res.is_none());
    }

    #[test]
    fn test_result_collect() {
        let res: [u32; 4] = ["1", "2", "3", "4"]
            .into_static_iter()
            .map(u32::from_str)
            .collect::<Result<_, _>>()
            .unwrap();
        assert_eq!(res, [1, 2, 3, 4]);

        let res: Result<[u32; 4], _> = ["1", "2", "-3", "4"]
            .into_static_iter()
            .map(u32::from_str)
            .collect();
        assert!(res.is_err());
    }

    #[test]
    fn test_enumerate() {
        let res = [5, 9, 1, 4]
            .into_static_iter()
            .enumerate()
            .fold(0, |acc, (idx, _)| {
                assert_eq!(idx, acc);
                acc + 1
            });
        assert_eq!(res, 4);
    }

    #[test]
    fn test_any() {
        let res = [1, 3, 5, 7].into_static_iter().any(|v| v % 2 == 0);
        assert!(!res);

        let res = [1, 3, 4, 7].into_static_iter().any(|v| v % 2 == 0);
        assert!(res);
    }

    #[test]
    fn test_all() {
        let res = [1, 3, 5, 7].into_static_iter().all(|v| v % 2 == 1);
        assert!(res);

        let res = [1, 3, 4, 7].into_static_iter().all(|v| v % 2 == 1);
        assert!(!res);
    }
}
