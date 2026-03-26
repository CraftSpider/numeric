//! Static length iterator adapters

use super::StaticIter;

/// See [`StaticIter::map`]
pub struct Map<T, F> {
    pub(crate) inner: T,
    pub(crate) func: F,
}

impl<I, T, M, const N: usize> StaticIter<N> for Map<I, M>
where
    I: StaticIter<N>,
    M: FnMut(I::Item) -> T,
{
    type Item = T;

    #[inline]
    unsafe fn idx(&mut self, idx: usize) -> Self::Item {
        (self.func)(self.inner.idx(idx))
    }
}

/// See [`StaticIter::zip`]
pub struct Zip<I1, I2> {
    pub(crate) left: I1,
    pub(crate) right: I2,
}

impl<I1, I2, const N: usize> StaticIter<N> for Zip<I1, I2>
where
    I1: StaticIter<N>,
    I2: StaticIter<N>,
{
    type Item = (I1::Item, I2::Item);

    #[inline]
    unsafe fn idx(&mut self, idx: usize) -> Self::Item {
        (self.left.idx(idx), self.right.idx(idx))
    }
}

/// See [`StaticIter::enumerate`]
pub struct Enumerate<I> {
    pub(crate) inner: I,
}

impl<I, const N: usize> StaticIter<N> for Enumerate<I>
where
    I: StaticIter<N>,
{
    type Item = (usize, I::Item);

    #[inline]
    unsafe fn idx(&mut self, idx: usize) -> Self::Item {
        (idx, self.inner.idx(idx))
    }
}

/// See [`StaticIter::cloned`]
pub struct Cloned<I> {
    pub(crate) inner: I,
}

impl<'a, I, T, const N: usize> StaticIter<N> for Cloned<I>
where
    I: StaticIter<N, Item = &'a T>,
    T: Clone + 'a,
{
    type Item = T;

    unsafe fn idx(&mut self, idx: usize) -> Self::Item {
        self.inner.idx(idx).clone()
    }
}

/// See [`StaticIter::copied`]
pub struct Copied<I> {
    pub(crate) inner: I,
}

impl<'a, I, T, const N: usize> StaticIter<N> for Copied<I>
where
    I: StaticIter<N, Item = &'a T>,
    T: Copy + 'a,
{
    type Item = T;

    unsafe fn idx(&mut self, idx: usize) -> Self::Item {
        *self.inner.idx(idx)
    }
}

/// See [`StaticIter::rev`]
pub struct Rev<I> {
    pub(crate) inner: I,
}

impl<I, const N: usize> StaticIter<N> for Rev<I>
where
    I: StaticIter<N>,
{
    type Item = I::Item;

    unsafe fn idx(&mut self, idx: usize) -> Self::Item {
        self.inner.idx(N - 1 - idx)
    }
}

/// See [`StaticIter::rev`]
pub struct Take<I, const N: usize, const M: usize> {
    pub(crate) inner: I,
}

impl<I, const N: usize, const M: usize> StaticIter<M> for Take<I, N, M>
where
    I: StaticIter<N>,
{
    type Item = I::Item;

    unsafe fn idx(&mut self, idx: usize) -> Self::Item {
        const { assert!(M <= N) };
        self.inner.idx(idx)
    }
}
