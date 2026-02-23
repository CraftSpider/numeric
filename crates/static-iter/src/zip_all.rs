//! Support for zipping up N many static length iterators into one.

use crate::{IntoStaticIter, StaticIter};

/// Trait for iterators that can be zipped into one.
pub trait ZipAll<const N: usize> {
    /// Item type of the resulting iterator.
    type Item;
    /// The iterator type that will be returned.
    type Iter: StaticIter<N, Item = Self::Item>;

    /// Convert this value into a zipped iterator. See [`zip_all`] for more details.
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

/// Iterator over an array of static length iterators. N iterators, each of length M.
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

impl<T, U, V, const N: usize> ZipAll<N> for (T, U, V)
where
    T: IntoStaticIter<N>,
    U: IntoStaticIter<N>,
    V: IntoStaticIter<N>,
{
    type Item = (T::Item, U::Item, V::Item);
    type Iter = TupleZipIter<(T::Iter, U::Iter, V::Iter)>;

    fn into_zip_iter(self) -> Self::Iter {
        TupleZipIter {
            iters: (
                self.0.into_static_iter(),
                self.1.into_static_iter(),
                self.2.into_static_iter(),
            ),
        }
    }
}

/// Iterator over a tuple of static length iterators.
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

impl<T, U, V, const N: usize> StaticIter<N> for TupleZipIter<(T, U, V)>
where
    T: StaticIter<N>,
    U: StaticIter<N>,
    V: StaticIter<N>,
{
    type Item = (T::Item, U::Item, V::Item);

    unsafe fn idx(&mut self, idx: usize) -> Self::Item {
        (
            self.iters.0.idx(idx),
            self.iters.1.idx(idx),
            self.iters.2.idx(idx),
        )
    }
}

/// Zip up many static length iterators, all of the same length, into one. The resulting iterator
/// will return one value from each of the zipped iterators for each iteration.
pub fn zip_all<T: ZipAll<N>, const N: usize>(val: T) -> T::Iter {
    val.into_zip_iter()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zip_arrays() {
        assert_eq!(
            zip_all([[1, 2], [3, 4], [5, 6], [7, 8]]).collect::<[_; _]>(),
            [[1, 3, 5, 7], [2, 4, 6, 8]]
        );

        assert_eq!(
            zip_all(([1u8, 2u8], [true, false], ["a", "b"])).collect::<[_; _]>(),
            [(1, true, "a"), (2, false, "b")],
        )
    }
}
