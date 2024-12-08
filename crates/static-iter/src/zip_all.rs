use crate::{IntoStaticIter, StaticIter};

pub trait ZipAll<const N: usize> {
    type Item;
    type Iter: StaticIter<N, Item = Self::Item>;

    fn into_zip_iter(self) -> Self::Iter;
}

impl<T, const N: usize, const M: usize> ZipAll<M> for [T; N]
where
    T: IntoStaticIter<M>,
{
    type Item = [T::Item; N];
    type Iter = ArrayZipIter<T::Iter, N, M>;

    fn into_zip_iter(self) -> Self::Iter {
        ArrayZipIter {
            arrs: self.map(T::into_static_iter),
        }
    }
}

pub struct ArrayZipIter<T, const N: usize, const M: usize> {
    arrs: [T; N],
}

impl<T, const N: usize, const M: usize> StaticIter<M> for ArrayZipIter<T, N, M>
where
    T: StaticIter<M>,
{
    type Item = [T::Item; N];

    unsafe fn idx(&mut self, idx: usize) -> Self::Item {
        self.arrs.each_mut().map(|val| val.idx(idx))
    }
}

impl<T, U, const N: usize> ZipAll<N> for (T, U)
where
    T: IntoStaticIter<N>,
    U: IntoStaticIter<N>,
{
    type Item = (T::Item, U::Item);
    type Iter = TupleZipIter<(T::Iter, U::Iter)>;

    fn into_zip_iter(self) -> Self::Iter {
        TupleZipIter {
            iters: (self.0.into_static_iter(), self.1.into_static_iter()),
        }
    }
}

pub struct TupleZipIter<T> {
    iters: T,
}

impl<T, U, const N: usize> StaticIter<N> for TupleZipIter<(T, U)>
where
    T: StaticIter<N>,
    U: StaticIter<N>,
{
    type Item = (T::Item, U::Item);

    unsafe fn idx(&mut self, idx: usize) -> Self::Item {
        (self.iters.0.idx(idx), self.iters.1.idx(idx))
    }
}

pub fn zip_all<T: ZipAll<N>, const N: usize>(val: T) -> T::Iter {
    val.into_zip_iter()
}
