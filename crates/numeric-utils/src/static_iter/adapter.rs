use crate::static_iter::StaticIter;

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
