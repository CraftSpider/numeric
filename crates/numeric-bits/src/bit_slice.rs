//! A type for bitwise operations on slices of integers

use std::mem;
use numeric_traits::identity::{One, Zero};
use numeric_traits::class::{BoundedBit, Integral};
use numeric_traits::ops::overflowing::OverflowingOps;

pub mod algos;
mod iter;

pub use iter::*;
use numeric_traits::ops::core::{BitAssignOps, NumAssignOps};
use numeric_traits::ops::widening::WideningMul;

pub trait BitLike: Integral + NumAssignOps + BitAssignOps + BoundedBit + OverflowingOps + WideningMul + Ord + Copy {
    const BIT_LEN: usize;
}
impl<T: Integral + NumAssignOps + BitAssignOps + BoundedBit + OverflowingOps + WideningMul + Ord + Copy> BitLike for T {
    const BIT_LEN: usize = mem::size_of::<T>() * 8;
}

pub trait BitSliceExt {
    type Bit: BitLike;

    /// Access this items as a slice of its elements
    fn slice(&self) -> &[Self::Bit];

    /// Access this item as a mutable slice of its inner elements
    fn slice_mut(&mut self) -> &mut [Self::Bit];

    #[inline]
    fn idx_bit(idx: usize) -> (usize, usize) {
        (
            idx / (mem::size_of::<Self::Bit>() * 8),
            idx % (mem::size_of::<Self::Bit>() * 8),
        )
    }

    /// Get the length of this slice in terms of [`Self::Bit`]
    #[inline]
    fn len(&self) -> usize {
        self.slice().len()
    }

    /// Whether this slice is empty
    #[inline]
    fn is_empty(&self) -> bool {
        self.slice().is_empty()
    }

    /// Get the length of this slice in bits
    #[inline]
    fn bit_len(&self) -> usize {
        self.len() * Self::Bit::BIT_LEN
    }

    /// Get the value of an element at a given index, panicking if the index is out of range
    fn get(&self, idx: usize) -> Self::Bit {
        self.get_opt(idx).expect("get index in-bounds")
    }

    /// Get the value of an element at a given index, returning `None` if the index is out of
    /// range
    fn get_opt(&self, idx: usize) -> Option<Self::Bit> {
        self.slice().get(idx).copied()
    }

    /// Get the value of a bit at a given position, panicking if the index is out of range
    ///
    /// # Panics
    ///
    /// If `idx` is outside the range of this slice
    fn get_bit(&self, idx: usize) -> bool {
        self.get_bit_opt(idx).expect("get_bit index in-bounds")
    }

    /// Get the value of a bit at a given index, returning `None` if the index is out of range
    fn get_bit_opt(&self, idx: usize) -> Option<bool> {
        let (idx, bit) = Self::idx_bit(idx);
        self.slice().get(idx).copied().map(|val| {
            val & (<Self::Bit as One>::one() << bit) != <Self::Bit as Zero>::zero()
        })
    }

    /// Set a single value by index on this slice, panicking if the index is out of range
    ///
    /// # Panics
    ///
    /// If `idx` is outside the range of this slice
    fn set(&mut self, idx: usize, val: Self::Bit) {
        self.set_opt(idx, val).unwrap_or_else(|| {
            panic!("Attempt to write value at index {} out of bounds", idx)
        })
    }

    /// Set a single value by index on this slice, returning `None` if the index is out of range
    #[must_use]
    fn set_opt(&mut self, idx: usize, val: Self::Bit) -> Option<()> {
        self.slice_mut().get_mut(idx).map(|cur| {
            *cur = val;
        })
    }

    /// Set a single bit by index on this slice, panicking if the index is out of range
    ///
    /// # Panics
    ///
    /// If `idx` is outside the range of this slice
    fn set_bit(&mut self, idx: usize, val: bool) {
        self.set_bit_opt(idx, val).unwrap_or_else(|| {
            let (idx, bit) = Self::idx_bit(idx);
            panic!("Attempt to write bit at {}:{} out of bounds", idx, bit)
        });
    }

    /// Set a single bit by index on this slice, returning `None` if the index is out of range
    fn set_bit_opt(&mut self, idx: usize, val: bool) -> Option<()> {
        let (idx, bit) = Self::idx_bit(idx);
        self.slice_mut()
            .get_mut(idx)
            .map(|item| {
                *item &= !(Self::Bit::one() << bit);
                if val {
                    *item |= Self::Bit::one() << bit;
                }
            })
    }

    /// Set a single value by index on this slice, doing nothing if the index is out of range
    #[inline]
    fn set_ignore(&mut self, pos: usize, val: Self::Bit) {
        let _ = self.set_opt(pos, val);
    }

    /// Set a single bit by index on this slice, doing nothing if the index is out of range
    #[inline]
    fn set_bit_ignore(&mut self, pos: usize, val: bool) {
        let _ = self.set_bit_opt(pos, val);
    }

    /// Get an iterator over the bit values of this slice
    fn iter_bits(&self) -> BitIter<'_, Self::Bit> {
        BitIter::new(self.slice())
    }
}

impl<I: BitLike> BitSliceExt for [I] {
    type Bit = I;

    #[inline]
    fn slice(&self) -> &[Self::Bit] {
        self
    }

    #[inline]
    fn slice_mut(&mut self) -> &mut [Self::Bit] {
        self
    }
}

impl<I: BitLike, const N: usize> BitSliceExt for [I; N] {
    type Bit = I;

    #[inline]
    fn slice(&self) -> &[Self::Bit] {
        self
    }

    #[inline]
    fn slice_mut(&mut self) -> &mut [Self::Bit] {
        self
    }
}

impl<I: BitLike> BitSliceExt for Vec<I> {
    type Bit = I;

    #[inline]
    fn slice(&self) -> &[Self::Bit] {
        self
    }

    #[inline]
    fn slice_mut(&mut self) -> &mut [Self::Bit] {
        self
    }
}

pub trait BitVecExt: BitSliceExt {
    fn extend(&mut self, len: usize, val: Self::Bit);

    /// Set a single value by index on this slice, extending it if the index is out of range
    fn set_push(&mut self, idx: usize, val: Self::Bit) {
        self.extend(idx, Self::Bit::zero());
        self.set_ignore(idx, val);
    }

    /// Set a single bit by index on this slice, extending it if the index is out of range
    fn set_bit_push(&mut self, idx: usize, val: bool) {
        let (len, _) = Self::idx_bit(idx);
        self.extend(len, Self::Bit::zero());
        self.set_bit_ignore(idx, val);
    }
}

impl<I: BitLike> BitVecExt for Vec<I> {
    fn extend(&mut self, len: usize, val: Self::Bit) {
        self.resize(len, val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idx() {
        for i in 0..8 {
            for j in 0..8 {
                assert_eq!(<[u8]>::idx_bit(i * 8 + j), (i, j));
            }
        }
    }

    #[test]
    fn test_get_bit() {
        let slice = &[0b1010101010101010u16, 0b1010101010101010];
        for idx in 0..32 {
            let b = slice.get_bit(idx);
            assert_eq!(b, (idx % 2) != 0);
        }
    }

    #[test]
    fn test_set_bit() {
        let mut data = [0b1010101010101010u16, 0b1010101010101010];
        let slice = &mut data;
        slice.set_bit(0, true);
        slice.set_bit(31, false);
        assert_eq!(slice, &[0b1010101010101011, 0b0010101010101010])
    }

    /*#[test]
    fn test_add_shift_mul_bitwise() {
        let slice1 = BitSlice::<&[u8], _>::new(&[0b00000000]);
        let slice2 = BitSlice::<&[u8], _>::new(&[0b00000001]);

        assert_eq!(BitSlice::add_shift_mul_bitwise(slice1, slice2).inner(), &[0b0]);

        let slice3 = BitSlice::<&[u8], _>::new(&[0b00000001]);
        let slice4 = BitSlice::<&[u8], _>::new(&[0b00000010]);

        assert_eq!(BitSlice::add_shift_mul_bitwise(slice3, slice4).inner(), &[0b10]);

        let slice5 = BitSlice::<&[u8], _>::new(&[0b00000010]);
        let slice6 = BitSlice::<&[u8], _>::new(&[0b00000010]);

        assert_eq!(BitSlice::add_shift_mul_bitwise(slice5, slice6).inner(), &[0b100]);
    }

    #[test]
    fn test_mul() {
        let slice1 = BitSlice::<&[u8], _>::new(&[0b00000001]);
        let slice2 = BitSlice::<&[u8], _>::new(&[0b00000001]);

        assert_eq!(BitSlice::add_shift_mul_bitwise(slice1.clone(), slice2.clone()).inner(), &[0b00000001]);
        assert_eq!(BitSlice::mul_long_element(slice1.clone(), slice2.clone()).inner(), &[0b00000001]);

        let slice3 = BitSlice::<&[u8], _>::new(&[0b10]);
        let slice4 = BitSlice::<&[u8], _>::new(&[0b10]);

        assert_eq!(BitSlice::add_shift_mul_bitwise(slice3.clone(), slice4.clone()).inner(), &[0b00000100]);
        assert_eq!(BitSlice::mul_long_element(slice3.clone(), slice4.clone()).inner(), &[0b00000100]);
    }*/

    #[test]
    fn test_div() {
        // TODO: Move this test into proper submodules
        use super::algos::BitwiseOps;

        let slice1: &[u8] = &[0b10];
        let slice2: &[u8] = &[0b01];

        assert_eq!(BitwiseOps::div_long(slice1, slice2).0, &[0b10]);

        let slice3: &[u8] = &[0b10];
        let slice4: &[u8] = &[0b10];

        assert_eq!(BitwiseOps::div_long(slice3, slice4).0, &[0b01]);

        let slice5: &[u8] = &[0b00000000, 0b1];
        let slice6: &[u8] = &[0b00000010];

        assert_eq!(BitwiseOps::div_long(slice5, slice6).0, &[0b10000000, 0b0]);

        let slice7: &[u8] = &[0b0, 0b0, 0b0, 0b1];
        let slice8: &[u8] = &[0b10];

        assert_eq!(BitwiseOps::div_long(slice7, slice8).0, &[0b0, 0b0, 0b10000000, 0b0]);
    }

    /*#[test]
    fn test_div_wrapping() {
        let slice1 = BitSlice::<[u8; 1], _>::new([0b10]);
        let slice2 = BitSlice::<&[u8], _>::new(&[0b01]);

        assert_eq!(BitSlice::div_long_element_wrapping(slice1, slice2).inner(), &[0b10]);

        let slice3 = BitSlice::<[u8; 1], _>::new([0b10]);
        let slice4 = BitSlice::<&[u8], _>::new(&[0b10]);

        assert_eq!(BitSlice::div_long_element_wrapping(slice3, slice4).inner(), &[0b01]);

        let slice5 = BitSlice::<[u8; 2], _>::new([0b00000000, 0b1]);
        let slice6 = BitSlice::<&[u8], _>::new(&[0b00000010]);

        assert_eq!(BitSlice::div_long_element_wrapping(slice5, slice6).inner(), &[0b10000000, 0b0]);

        let slice7 = BitSlice::<[u8; 4], _>::new([0b0, 0b0, 0b0, 0b1]);
        let slice8 = BitSlice::<&[u8], _>::new(&[0b10]);

        assert_eq!(BitSlice::div_long_element_wrapping(slice7, slice8).inner(), &[0b0, 0b0, 0b10000000, 0b0]);
    }

    #[test]
    fn test_rem() {
        for i in 0..4 {
            let slice = &[i];
            let slice1 = BitSlice::<&[u8], _>::new(slice);
            let slice2 = BitSlice::<&[u8], _>::new(&[0b10]);

            assert_eq!(BitSlice::div_long_bitwise(slice1, slice2).1.inner(), &[i % 2]);
        }

        for i in 0..6 {
            let slice = &[i];
            let slice3 = BitSlice::<&[u8], _>::new(slice);
            let slice4 = BitSlice::<&[u8], _>::new(&[0b11]);

            assert_eq!(BitSlice::div_long_bitwise(slice3, slice4).1.inner(), &[i % 3]);
        }

        let slice5 = BitSlice::<&[u8], _>::new(&[0b00000001, 0b111]);
        let slice6 = BitSlice::<&[u8], _>::new(&[0b00000010]);

        assert_eq!(BitSlice::div_long_bitwise(slice5, slice6).1.inner(), &[0b01]);
    }

    #[test]
    fn test_shl() {
        let slice = BitSlice::<&[u16], _>::new(&[0b1010101010101010, 0b1010101010101010]);
        let res = BitSlice::shl_bitwise(slice.clone(), 1);
        assert_eq!(res.inner(), &[0b0101010101010100, 0b0101010101010101, 0b1]);

        let res = BitSlice::shl_wrap_and_mask(slice, 1);
        assert_eq!(res.inner(), &[0b0101010101010100, 0b0101010101010101, 0b1]);
    }

    #[test]
    fn test_checked_shl() {
        let val = &mut [0b1010101010101010, 0b1010101010101010];
        let slice = BitSlice::<&mut [u16], _>::new(val);
        let res = BitSlice::shl_wrap_and_mask_checked(slice, 1);
        assert_eq!(res.map(|s| s.into_inner()), Some(&mut [0b0101010101010100u16, 0b0101010101010101] as &mut [_]));

        let val = &mut [0b1010101010101010, 0b1010101010101010];
        let slice = BitSlice::<&mut [u16], _>::new(val);
        let res = BitSlice::shl_wrap_and_mask_checked(slice, 33);
        assert_eq!(res, None);
    }

    #[test]
    fn test_wrapping_shl() {
        let val = &mut [0b1010101010101010, 0b1010101010101010];
        let slice = BitSlice::<&mut [u16], _>::new(val);
        let res = BitSlice::shl_wrap_and_mask_wrapping(slice, 1);
        assert_eq!(res.inner(), &[0b0101010101010100, 0b0101010101010101]);

        let val = &mut [0b1010101010101010, 0b1010101010101010];
        let slice = BitSlice::<&mut [u16], _>::new(val);
        let res = BitSlice::shl_wrap_and_mask_wrapping(slice, 33);
        assert_eq!(res.inner(), &[0b0101010101010100, 0b0101010101010101]);
    }

    #[test]
    fn test_shr() {
        let slice = BitSlice::<&[u16], _>::new(&[0b1010101010101010, 0b1010101010101010]);
        let res = BitSlice::shr_bitwise(slice.clone(), 1);
        assert_eq!(res.inner(), &[0b0101010101010101, 0b0101010101010101]);

        let res = BitSlice::shr_wrap_and_mask(slice, 1);
        assert_eq!(res.inner(), &[0b0101010101010101, 0b0101010101010101]);
    }

    #[test]
    fn test_checked_shr() {
        let val = &mut [0b1010101010101010, 0b1010101010101010];
        let slice = BitSlice::<&mut [u16], _>::new(val);
        let res = BitSlice::shr_wrap_and_mask_checked(slice, 1);
        assert_eq!(res.map(|s| s.into_inner()), Some(&mut [0b0101010101010101u16, 0b0101010101010101] as &mut [_]));

        let val = &mut [0b1010101010101010, 0b1010101010101010];
        let slice = BitSlice::<&mut [u16], _>::new(val);
        let res = BitSlice::shr_wrap_and_mask_checked(slice, 33);
        assert_eq!(res, None);
    }

    #[test]
    fn test_wrapping_shr() {
        let val = &mut [0b1010101010101010, 0b1010101010101010];
        let slice = BitSlice::<&mut [u16], _>::new(val);
        let res = BitSlice::shr_wrap_and_mask_wrapping(slice, 1);
        assert_eq!(res.inner(), &[0b0101010101010101, 0b0101010101010101]);

        let val = &mut [0b1010101010101010, 0b1010101010101010];
        let slice = BitSlice::<&mut [u16], _>::new(val);
        let res = BitSlice::shr_wrap_and_mask_wrapping(slice, 33);
        assert_eq!(res.inner(), &[0b0101010101010101, 0b0101010101010101]);
    }*/
}
