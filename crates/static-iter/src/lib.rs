#![no_std]

use adapter::{Enumerate, Map, Zip};
use core::convert::Infallible;
use core::mem;
use core::mem::MaybeUninit;
use core::ops::{Add, ControlFlow, Mul};
use numeric_traits::identity::{One, Zero};

pub trait FromStaticIter<T, const N: usize>: Sized {
    type Uninit;
    type Break;
    fn uninit() -> Self::Uninit;
    fn write(this: Self::Uninit, idx: usize, val: T) -> ControlFlow<Self::Break, Self::Uninit>;

    /// # Safety
    ///
    /// This will be called with uninit data in two cases:
    /// 1) The underlying iterator has been polled to completion, and `write` called once for each
    ///    index
    /// 2) Write returned `ControlFlow::Break` at any point
    unsafe fn finish(this: ControlFlow<Self::Break, Self::Uninit>) -> Self;

    fn from_static_iter(mut iter: impl StaticIter<N, Item = T>) -> Self {
        let uninit = (0..N).try_fold(Self::uninit(), |acc, idx| {
            let val = unsafe { iter.idx(idx) };
            Self::write(acc, idx, val)
        });
        unsafe { Self::finish(uninit) }
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
        unsafe { mem::transmute_copy(&c) }
    }

    fn from_static_iter(mut iter: impl StaticIter<N, Item = T>) -> Self {
        core::array::from_fn(|idx| unsafe { iter.idx(idx) })
    }
}

// TODO: Figure out how to make this generic over the inner collection
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

pub trait StaticIter<const N: usize>: Sized {
    type Item;

    /// # Safety
    ///
    /// This function must be called at most once for each index in order starting from zero
    /// The index must be in the range 0..N
    unsafe fn idx(&mut self, idx: usize) -> Self::Item;

    #[inline]
    fn map<T, F: FnMut(Self::Item) -> T>(self, func: F) -> Map<Self, F> {
        Map { inner: self, func }
    }

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

    #[inline]
    fn enumerate(self) -> Enumerate<Self> {
        Enumerate { inner: self }
    }

    #[inline]
    fn fold<T, F: FnMut(T, Self::Item) -> T>(mut self, start: T, mut func: F) -> T {
        (0..N).fold(start, |acc, idx| {
            // SAFETY: Follows contract of `idx` - we call exactly once for each value from `0..N`
            let item = unsafe { self.idx(idx) };
            func(acc, item)
        })
    }

    // TODO: This really wants to use `Try`
    #[inline]
    fn try_fold<T, E, F: FnMut(T, Self::Item) -> Result<T, E>>(
        mut self,
        start: T,
        mut func: F,
    ) -> Result<T, E> {
        (0..N).try_fold(start, |acc, idx| {
            // SAFETY: Follows contract of `idx` - we call exactly once for each value from `0..N`
            let item = unsafe { self.idx(idx) };
            func(acc, item)
        })
    }

    fn collect<C: FromStaticIter<Self::Item, N>>(self) -> C {
        C::from_static_iter(self)
    }

    fn any<F: FnMut(Self::Item) -> bool>(self, mut func: F) -> bool {
        self.try_fold((), |(), x| if func(x) { Err(()) } else { Ok(()) }) == Err(())
    }

    fn all<F: FnMut(Self::Item) -> bool>(self, mut func: F) -> bool {
        self.try_fold((), |(), x| if func(x) { Ok(()) } else { Err(()) }) == Ok(())
    }

    // TODO: Move this and sum to an extension in numeric-traits? Makes static_iter stand alone
    fn sum(self) -> Self::Item
    where
        Self::Item: Zero + Add<Output = Self::Item>,
    {
        self.fold(Self::Item::zero(), |acc, val| acc + val)
    }

    fn product(self) -> Self::Item
    where
        Self::Item: One + Mul<Output = Self::Item>,
    {
        self.fold(Self::Item::one(), |acc, val| acc * val)
    }
}

pub trait IntoStaticIter<const N: usize> {
    type Item;
    type Iter: StaticIter<N, Item = Self::Item>;

    fn into_static_iter(self) -> Self::Iter;
}

impl<I: StaticIter<N>, const N: usize> IntoStaticIter<N> for I {
    type Item = I::Item;
    type Iter = I;

    fn into_static_iter(self) -> Self::Iter {
        self
    }
}

pub struct StaticRangeTo<const N: usize>;

impl<const N: usize> IntoStaticIter<N> for StaticRangeTo<N> {
    type Item = usize;
    type Iter = StaticRangeToIter<N>;

    fn into_static_iter(self) -> Self::Iter {
        StaticRangeToIter::<N>(())
    }
}

pub struct StaticRangeToIter<const N: usize>(());

impl<const N: usize> StaticIter<N> for StaticRangeToIter<N> {
    type Item = usize;

    unsafe fn idx(&mut self, idx: usize) -> Self::Item {
        idx
    }
}

pub mod adapter;
pub mod array;
pub mod codegen;
pub mod zip_all;

pub use zip_all::zip_all;

#[cfg(test)]
mod tests {
    use super::*;

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
}
