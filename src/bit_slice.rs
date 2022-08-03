use std::cmp::Ordering;
use std::fmt;
use std::hint::unreachable_unchecked;
use std::ops::{Add, Div, Mul, Neg, Shl, Shr, Sub};
use num_traits::{PrimInt, One, Zero};

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
        for idx in 0..self.bit_len() {
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
        for idx in (0..self.bit_len()).rev() {
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
        let mut out = BitSlice::new(vec![<S::Output>::zero(); self.len()]);
        for idx in 0..self.bit_len() {
            let new = self.get_bit(idx);
            #[allow(clippy::suspicious_arithmetic_impl)]
            out.set_bit_pushing(idx + rhs, new);
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
        let len = usize::max(self.len(), rhs.len());
        let bit_len = usize::max(self.bit_len(), rhs.bit_len());
        let mut out = BitSlice::new(vec![<S::Output>::zero(); len]);

        let mut carry = false;
        for idx in 0..bit_len {
            let l = self.get_bit(idx) as u8;
            let r = rhs.get_bit(idx) as u8;

            let c = if carry {
                carry = false;
                1
            } else {
                0
            };

            let new = match c + l + r {
                0 => false,
                1 => true,
                2 => {
                    carry = true;
                    false
                }
                3 => {
                    carry = true;
                    true
                }
                _ => unsafe { unreachable_unchecked() },
            };

            out.set_bit_pushing(idx, new);
        }

        out
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
    type Output = (BitSlice<Vec<S::Output>>, BitSlice<Vec<S::Output>>);

    fn div(self, rhs: BitSlice<T>) -> Self::Output {
        let num = self;
        let div = rhs;

        let len = usize::max(num.len(), div.len());
        let bit_len = usize::max(num.bit_len(), div.bit_len());

        let mut quotient = BitSlice::new(vec![<S::Output>::zero(); len]);
        let mut remainder: BitSlice<_> = BitSlice::new(vec![<S::Output>::zero(); len]);

        for idx in (0..bit_len).rev() {
            remainder = remainder << 1;
            remainder.set_bit(0, num.get_bit(idx));
            if remainder >= div {
                // Ignore the bool - subtract will never overflow
                remainder = (remainder - div.clone()).0;
                quotient.set_bit(idx, true);
            }
        }

        (quotient, remainder)
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
