use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Rem, Shl, Shr, Sub};
use num_traits::{PrimInt, One, Zero};

mod algos;

mod private {
    use std::slice::SliceIndex;

    pub trait Len {
        fn len(&self) -> usize;

        fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    impl<T> Len for [T] {
        fn len(&self) -> usize {
            <[T]>::len(self)
        }
    }

    impl<T> Len for Vec<T> {
        fn len(&self) -> usize {
            <Vec<T>>::len(self)
        }
    }

    impl<T: ?Sized> Len for &T
    where
        T: Len
    {
        fn len(&self) -> usize {
            T::len(*self)
        }
    }

    impl<T: ?Sized> Len for &mut T
    where
        T: Len
    {
        fn len(&self) -> usize {
            T::len(*self)
        }
    }

    pub trait IndexOpt<T> {
        type Output: ?Sized;

        fn index(&self, idx: T) -> Option<&Self::Output>;
    }

    impl<T, I> IndexOpt<I> for [T]
    where
        I: SliceIndex<[T]>,
    {
        type Output = I::Output;

        fn index(&self, idx: I) -> Option<&Self::Output> {
            self.get(idx)
        }
    }

    impl<T, I> IndexOpt<I> for &[T]
    where
        I: SliceIndex<[T]>,
    {
        type Output = I::Output;

        fn index(&self, idx: I) -> Option<&Self::Output> {
            self.get(idx)
        }
    }

    impl<T, I> IndexOpt<I> for &mut [T]
    where
        I: SliceIndex<[T]>,
    {
        type Output = I::Output;

        fn index(&self, idx: I) -> Option<&Self::Output> {
            self.get(idx)
        }
    }
    
    impl<T, I> IndexOpt<I> for Vec<T>
    where
        I: SliceIndex<[T]>,
    {
        type Output = I::Output;

        fn index(&self, idx: I) -> Option<&Self::Output> {
            self.get(idx)
        }
    }

    pub trait IndexOptMut<T>: IndexOpt<T> {
        fn index_mut(&mut self, idx: T) -> Option<&mut Self::Output>;
    }

    impl<T, I> IndexOptMut<I> for [T]
    where
        I: SliceIndex<[T]>,
    {
        fn index_mut(&mut self, idx: I) -> Option<&mut Self::Output> {
            self.get_mut(idx)
        }
    }

    impl<T, I> IndexOptMut<I> for &mut [T]
    where
        I: SliceIndex<[T]>,
    {
        fn index_mut(&mut self, idx: I) -> Option<&mut Self::Output> {
            self.get_mut(idx)
        }
    }

    impl<T, I> IndexOptMut<I> for Vec<T>
    where
        I: SliceIndex<[T]>,
    {
        fn index_mut(&mut self, idx: I) -> Option<&mut Self::Output> {
            self.get_mut(idx)
        }
    }
}

use private::*;

/// Utility for bit-by-bit algorithms on slices of primitive integers
pub struct BitSlice<S>(S);

impl<S> BitSlice<S> {
    pub fn new(inner: S) -> BitSlice<S> {
        BitSlice(inner)
    }

    pub fn inner(&self) -> &S {
        &self.0
    }

    pub fn into_inner(self) -> S {
        self.0
    }
}

impl<S> BitSlice<S>
where
    S: Len + IndexOpt<usize>,
    S::Output: Sized,
{
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn bit_len(&self) -> usize {
        self.len() * std::mem::size_of::<S::Output>() * 8
    }
}

impl<S> BitSlice<S>
where
    S: IndexOpt<usize>,
    S::Output: PrimInt,
{
    fn idx_bit(&self, pos: usize) -> (usize, usize) {
        (
            pos / (std::mem::size_of::<S::Output>() * 8),
            pos % (std::mem::size_of::<S::Output>() * 8),
        )
    }

    pub fn get_bit(&self, pos: usize) -> bool {
        let (idx, bit) = self.idx_bit(pos);
        let val = self.0.index(idx).copied().unwrap_or_else(<S::Output>::zero);
        val & (<S::Output>::one() << bit) != <S::Output>::zero()
    }
}

impl<S> BitSlice<S>
where
    S: IndexOptMut<usize>,
    S::Output: PrimInt,
{
    /// Set a single bit by index on this slice
    ///
    /// # Panics
    ///
    /// If `pos` is outside the range of this slice
    pub fn set_bit(&mut self, pos: usize, val: bool) {
        let (idx, bit) = self.idx_bit(pos);
        if let Some(item) = self.0.index_mut(idx) {
            *item = *item & !(<S::Output>::one() << bit);
            if val {
                *item = *item | (<S::Output>::one() << bit);
            }
        } else {
            panic!("Attempt to write bit at {}:{} out of bounds", idx, bit);
        }
    }

    pub fn set_bit_ignore(&mut self, pos: usize, val: bool) {
        let (idx, bit) = self.idx_bit(pos);
        if let Some(item) = self.0.index_mut(idx) {
            *item = *item & !(<S::Output>::one() << bit);
            if val {
                *item = *item | (<S::Output>::one() << bit);
            }
        }
    }
}

impl<T> BitSlice<Vec<T>> {
    #[must_use]
    pub fn as_slice(&self) -> BitSlice<&[T]> {
        BitSlice(&self.0)
    }

    #[must_use]
    pub fn as_mut_slice(&mut self) -> BitSlice<&mut [T]> {
        BitSlice(&mut self.0)
    }
}

impl<T> BitSlice<Vec<T>>
where
    T: PrimInt,
{
    pub fn set_bit_pushing(&mut self, pos: usize, val: bool) {
        let (idx, bit) = self.idx_bit(pos);

        let item = loop {
            match self.0.index_mut(idx) {
                Some(item) => break item,
                None => self.0.push(T::zero()),
            }
        };

        *item = *item & !(T::one() << bit);
        if val {
            *item = *item | (T::one() << bit);
        }
    }
}

impl<T> BitSlice<&mut [T]> {
    #[must_use]
    pub fn as_slice(&self) -> BitSlice<&[T]> {
        BitSlice(self.0)
    }
}

impl<S> fmt::Debug for BitSlice<S>
where
    S: IndexOpt<usize> + Len,
    S::Output: PrimInt,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for idx in (0..self.bit_len()).rev() {
            write!(f, "{}", self.get_bit(idx) as u8)?;
        }
        Ok(())
    }
}

impl<S> Clone for BitSlice<S>
where
    S: Clone,
{
    fn clone(&self) -> Self {
        BitSlice(self.0.clone())
    }
}

impl<S> Shr<usize> for BitSlice<S>
where
    S: IndexOpt<usize> + Len,
    S::Output: PrimInt,
{
    type Output = BitSlice<Vec<S::Output>>;

    fn shr(self, rhs: usize) -> Self::Output {
        let mut out = BitSlice::new(vec![<S::Output>::zero(); self.len()]);
        for idx in (0..=self.bit_len()).rev() {
            let new = self.get_bit(idx);
            if let Some(idx) = idx.checked_sub(rhs) {
                out.set_bit_ignore(idx, new);
            }
        }
        out
    }
}

impl<S> Shl<usize> for BitSlice<S>
where
    S: IndexOpt<usize> + Len,
    S::Output: PrimInt,
{
    type Output = BitSlice<Vec<S::Output>>;

    fn shl(self, rhs: usize) -> Self::Output {
        let bit_len = self.bit_len();
        let mut out = BitSlice::new(vec![<S::Output>::zero(); self.len()]);
        for idx in 0..=bit_len {
            let new = self.get_bit(idx);
            if new || idx + rhs < bit_len {
                #[allow(clippy::suspicious_arithmetic_impl)]
                out.set_bit_pushing(idx + rhs, new);
            }
        }
        out
    }
}

impl<S> Neg for BitSlice<S>
where
    S: IndexOptMut<usize> + Len,
    S::Output: PrimInt,
{
    type Output = BitSlice<S>;

    fn neg(mut self) -> Self::Output {
        for idx in 0..self.bit_len() {
            self.set_bit(idx, !self.get_bit(idx));
        }
        self
    }
}

impl<S, T> Add<BitSlice<T>> for BitSlice<S>
where
    S: IndexOpt<usize> + Len,
    T: IndexOpt<usize> + Len,
    S::Output: PrimInt,
    T::Output: PrimInt,
{
    type Output = BitSlice<Vec<S::Output>>;

    fn add(self, rhs: BitSlice<T>) -> Self::Output {
        Self::add_bitwise(self, rhs)
    }
}

impl<S, T> Sub<BitSlice<T>> for BitSlice<S>
where
    S: IndexOpt<usize> + Len,
    T: IndexOpt<usize> + Len,
    S::Output: PrimInt,
    T::Output: PrimInt,
{
    type Output = (BitSlice<Vec<S::Output>>, bool);

    fn sub(self, rhs: BitSlice<T>) -> Self::Output {
        let len = usize::max(self.len(), rhs.len());
        let bit_len = usize::max(self.bit_len(), rhs.bit_len());
        let mut out = BitSlice::new(vec![<S::Output>::zero(); len]);

        let mut carry = false;
        for idx in 0..bit_len {
            let l = self.get_bit(idx);
            let r = rhs.get_bit(idx);

            let c = if carry {
                carry = false;
                true
            } else {
                false
            };

            let new = match (l, r, c) {
                (true, false, false) => true,
                (true, true, false) | (true, false, true) | (false, false, false) => false,
                (false, true, false) | (false, false, true) | (true, true, true) => {
                    carry = true;
                    true
                }
                (false, true, true) => {
                    carry = true;
                    false
                }
            };

            out.set_bit(idx, new);
        }

        if carry {
            out.set_bit(0, !out.get_bit(0));
            out = -out;
        }

        (out, carry)
    }
}

impl<S, T> Mul<BitSlice<T>> for BitSlice<S>
where
    S: IndexOpt<usize> + Len + Clone,
    T: IndexOpt<usize> + Len,
    S::Output: PrimInt,
    T::Output: PrimInt,
{
    type Output = BitSlice<Vec<S::Output>>;

    fn mul(self, rhs: BitSlice<T>) -> Self::Output {
        let len = usize::max(self.len(), rhs.len());
        let mut new_self = self << 0;
        let mut out = BitSlice::new(vec![<S::Output>::zero(); len * 2]);

        for idx in 0..rhs.bit_len() {
            let r = rhs.get_bit(idx);
            if r {
                out = out + new_self.clone();
            }
            new_self = new_self << 1;
        }

        out
    }
}

impl<S, T> Div<BitSlice<T>> for BitSlice<S>
where
    BitSlice<Vec<S::Output>>: PartialOrd<BitSlice<T>>,
    S: IndexOpt<usize> + Len,
    T: IndexOpt<usize> + Len + Clone,
    S::Output: PrimInt,
    T::Output: PrimInt,
{
    type Output = BitSlice<Vec<S::Output>>;

    fn div(self, rhs: BitSlice<T>) -> Self::Output {
        Self::long_div_bitwise(self, rhs).0
    }
}

impl<S, T> Rem<BitSlice<T>> for BitSlice<S>
where
    BitSlice<Vec<S::Output>>: PartialOrd<BitSlice<T>>,
    S: IndexOpt<usize> + Len,
    T: IndexOpt<usize> + Len + Clone,
    S::Output: PrimInt,
    T::Output: PrimInt,
{
    type Output = BitSlice<Vec<S::Output>>;

    fn rem(self, rhs: BitSlice<T>) -> Self::Output {
        Self::long_div_bitwise(self, rhs).1
    }
}

impl<S, T> PartialEq<BitSlice<T>> for BitSlice<S>
where
    S: PartialEq<T>,
{
    fn eq(&self, other: &BitSlice<T>) -> bool {
        self.0 == other.0
    }
}

impl<T, U> PartialOrd<BitSlice<&[U]>> for BitSlice<Vec<T>>
where
    T: PartialEq<U>,
    [T]: PartialOrd<[U]>,
{
    fn partial_cmp(&self, other: &BitSlice<&[U]>) -> Option<Ordering> {
        <[T] as PartialOrd<[U]>>::partial_cmp(&self.0, other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idx() {
        let slice = BitSlice::<&[u8]>::new(&[]);

        for i in 0..8 {
            for j in 0..8 {
                assert_eq!(slice.idx_bit(i * 8 + j), (i, j));
            }
        }
    }

    #[test]
    fn test_get_bit() {
        let slice = BitSlice::<&[u16]>::new(&[0b1010101010101010, 0b1010101010101010]);
        for idx in 0..32 {
            let b = slice.get_bit(idx);
            assert_eq!(b, (idx % 2) != 0);
        }
    }

    #[test]
    fn test_set_bit() {
        let mut data = [0b1010101010101010, 0b1010101010101010];
        let mut slice = BitSlice::<&mut [u16]>::new(&mut data);
        slice.set_bit(0, true);
        slice.set_bit(31, false);
        assert_eq!(slice.inner(), &[0b1010101010101011, 0b0010101010101010])
    }

    #[test]
    fn test_mul() {
        let slice1 = BitSlice::<&[u8]>::new(&[0b00000000]);
        let slice2 = BitSlice::<&[u8]>::new(&[0b00000001]);

        assert_eq!((slice1 * slice2).inner(), &[0b0, 0b0, 0b0]);

        let slice3 = BitSlice::<&[u8]>::new(&[0b00000001]);
        let slice4 = BitSlice::<&[u8]>::new(&[0b00000010]);

        assert_eq!((slice3 * slice4).inner(), &[0b10, 0b0, 0b0]);

        let slice5 = BitSlice::<&[u8]>::new(&[0b00000010]);
        let slice6 = BitSlice::<&[u8]>::new(&[0b00000010]);

        assert_eq!((slice5 * slice6).inner(), &[0b100, 0b0, 0b0]);
    }

    #[test]
    fn test_div() {
        let slice1 = BitSlice::<&[u8]>::new(&[0b10]);
        let slice2 = BitSlice::<&[u8]>::new(&[0b01]);

        assert_eq!((slice1 / slice2).inner(), &[0b10]);

        let slice3 = BitSlice::<&[u8]>::new(&[0b10]);
        let slice4 = BitSlice::<&[u8]>::new(&[0b10]);

        assert_eq!((slice3 / slice4).inner(), &[0b01]);

        let slice5 = BitSlice::<&[u8]>::new(&[0b00000000, 0b1]);
        let slice6 = BitSlice::<&[u8]>::new(&[0b00000010]);

        assert_eq!((slice5 / slice6).inner(), &[0b10000000, 0b0]);

        let slice7 = BitSlice::<&[u8]>::new(&[0b0, 0b0, 0b0, 0b1]);
        let slice8 = BitSlice::<&[u8]>::new(&[0b10]);

        assert_eq!((slice7 / slice8).inner(), &[0b0, 0b0, 0b10000000, 0b0]);
    }

    #[test]
    fn test_rem() {
        for i in 0..4 {
            let slice = &[i];
            let slice1 = BitSlice::<&[u8]>::new(slice);
            let slice2 = BitSlice::<&[u8]>::new(&[0b10]);

            assert_eq!((slice1 % slice2).inner(), &[i % 2]);
        }

        for i in 0..6 {
            let slice = &[i];
            let slice3 = BitSlice::<&[u8]>::new(slice);
            let slice4 = BitSlice::<&[u8]>::new(&[0b11]);

            assert_eq!((slice3 % slice4).inner(), &[i % 3]);
        }

        let slice5 = BitSlice::<&[u8]>::new(&[0b00000001, 0b111]);
        let slice6 = BitSlice::<&[u8]>::new(&[0b00000010]);

        assert_eq!((slice5 % slice6).inner(), &[0b01, 0b0]);
    }

    #[test]
    fn test_shl() {
        let slice = BitSlice::<&[u16]>::new(&[0b1010101010101010, 0b1010101010101010]);
        let res = slice << 1;
        assert_eq!(res.inner(), &[0b0101010101010100, 0b0101010101010101, 0b1]);
    }
}
