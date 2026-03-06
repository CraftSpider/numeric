use crate::algos::{Add, Algo, AssignAlgo, Bitwise, Element};
use crate::bit_slice::{BitLike, BitOwned, BitSlice};
use core::hint::unreachable_unchecked;
use numeric_traits::identity::{One, Zero};
use numeric_traits::ops::overflowing::OverflowingAdd;

impl Algo<Add> for Element {
    fn overflowing<O, L, R>(left: &L, right: &R) -> (O, bool)
    where
        O: BitOwned<Bit = L::Bit>,
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let mut out = O::zeroed(len + 1);
        let overflow = Self::overflowing_into(left, right, &mut out);
        (out, overflow)
    }

    fn overflowing_into<L, R, O>(left: &L, right: &R, out: &mut O) -> bool
    where
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
        O: BitOwned<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let zero = L::Bit::zero();
        let one = L::Bit::one();

        let mut overflow = false;
        let mut carry = false;

        for idx in 0..=len {
            let l = left.get(idx).unwrap_or(zero);
            let r = right.get(idx).unwrap_or(zero);

            let (res, new_carry) = l.overflowing_add(if carry { one } else { zero });
            carry = new_carry;

            let (res, new_carry) = res.overflowing_add(r);
            // As of Rust 1.86 nightly, this is faster than `carry |= new_carry`
            if new_carry {
                carry = true;
            }

            match out.set_opt(idx, res) {
                None if !res.is_zero() => overflow = true,
                _ => (),
            }
        }

        out.shrink();
        overflow
    }
}

impl AssignAlgo<Add> for Element {
    fn overflowing<O, L, R>(left: &mut L, right: &R) -> bool
    where
        O: BitOwned,
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let zero = L::Bit::zero();
        let one = L::Bit::one();

        let mut overflow = false;
        let mut carry = false;

        for idx in 0..=len {
            let l = left.get(idx).unwrap_or(zero);
            let r = right.get(idx).unwrap_or(zero);

            let (res, new_carry) = l.overflowing_add(if carry { one } else { zero });
            carry = new_carry;

            let (res, new_carry) = res.overflowing_add(r);
            // As of Rust 1.86 nightly, this is faster than `carry |= new_carry`
            if new_carry {
                carry = true;
            }

            match left.set_opt(idx, res) {
                None if !res.is_zero() => overflow = true,
                _ => (),
            }
        }

        overflow
    }
}

impl Algo<Add> for Bitwise {
    fn overflowing<O, L, R>(left: &L, right: &R) -> (O, bool)
    where
        O: BitOwned<Bit = L::Bit>,
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        let bit_len = usize::max(left.bit_len(), right.bit_len());
        let mut out = O::zeroed(bit_len / L::Bit::BIT_LEN + 1);
        let overflow = Self::overflowing_into(left, right, &mut out);
        (out, overflow)
    }

    fn overflowing_into<L, R, O>(left: &L, right: &R, out: &mut O) -> bool
    where
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
        O: BitOwned<Bit = L::Bit>,
    {
        let bit_len = usize::max(left.bit_len(), right.bit_len());

        let mut overflow = false;
        let mut carry = false;
        for idx in 0..=bit_len {
            let l = u8::from(left.get_bit(idx).unwrap_or(false));
            let r = u8::from(right.get_bit(idx).unwrap_or(false));

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
                // SAFETY: `c`, `l`, and `r` in range 0..=1, can't be a value beyond 3
                _ => unsafe { unreachable_unchecked() },
            };

            match out.set_bit_opt(idx, new) {
                None if new => overflow = true,
                _ => (),
            }
        }

        out.shrink();
        overflow
    }
}

impl AssignAlgo<Add> for Bitwise {
    fn overflowing<O, L, R>(left: &mut L, right: &R) -> bool
    where
        O: BitOwned,
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        let bit_len = usize::max(left.bit_len(), right.bit_len());

        let mut overflow = false;
        let mut carry = false;
        for idx in 0..=bit_len {
            let l = u8::from(left.get_bit(idx).unwrap_or(false));
            let r = u8::from(right.get_bit(idx).unwrap_or(false));

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
                // SAFETY: `c`, `l`, and `r` in range 0..=1, can't be a value beyond 3
                _ => unsafe { unreachable_unchecked() },
            };

            match left.set_bit_opt(idx, new) {
                None if new => overflow = true,
                _ => (),
            }
        }

        overflow
    }
}

#[cfg(test)]
mod tests {
    use crate::algos::{Add, Algo, Bitwise, Element};

    #[cfg(feature = "alloc")]
    fn test_long<B: Algo<Add>>() {
        use alloc::vec::Vec;
        // Simple addition
        assert_eq!(B::wrapping::<Vec<_>, _, _>(&[0u32], &[0]), &[0]);
        assert_eq!(B::wrapping::<Vec<_>, _, _>(&[0u32], &[1]), &[1]);
        assert_eq!(B::wrapping::<Vec<_>, _, _>(&[1u32], &[0]), &[1]);
        assert_eq!(B::wrapping::<Vec<_>, _, _>(&[1u32], &[1]), &[2]);

        // Long addition handled correctly
        assert_eq!(B::wrapping::<Vec<_>, _, _>(&[0u32], &[0, 1]), &[0, 1]);
        assert_eq!(B::wrapping::<Vec<_>, _, _>(&[1u32], &[0, 1]), &[1, 1]);
        assert_eq!(B::wrapping::<Vec<_>, _, _>(&[u32::MAX], &[1]), &[0, 1]);
    }

    fn test_wrapping<B: Algo<Add>>() {
        // Simple addition
        assert_eq!(B::wrapping::<[_; _], _, _>(&[0u32], &[0]), [0]);
        assert_eq!(B::wrapping::<[_; _], _, _>(&[0u32], &[1]), [1]);
        assert_eq!(B::wrapping::<[_; _], _, _>(&[1u32], &[0]), [1]);
        assert_eq!(B::wrapping::<[_; _], _, _>(&[1u32], &[1]), [2]);

        // Long addition handled correctly
        assert_eq!(B::wrapping::<[_; _], _, _>(&[0u32], &[0, 1]), [0, 1]);
        assert_eq!(B::wrapping::<[_; _], _, _>(&[1u32], &[0, 1]), [1, 1]);
        assert_eq!(B::wrapping::<[_; _], _, _>(&[u32::MAX], &[1]), [0, 1]);
    }

    fn test_saturating<B: Algo<Add>>() {
        assert_eq!(B::saturating::<[_; _], _, _>(&[0u32], &[0]), [0]);
        assert_eq!(B::saturating::<[_; _], _, _>(&[1u32], &[1]), [2]);
        assert_eq!(B::saturating::<[_; _], _, _>(&[1], &[u32::MAX]), [u32::MAX]);
        assert_eq!(B::saturating::<[_; _], _, _>(&[u32::MAX], &[1]), [u32::MAX]);
        assert_eq!(
            B::saturating::<[_; _], _, _>(&[u32::MAX], &[u32::MAX]),
            [u32::MAX]
        );
    }

    /// Test some edge cases of the desired AddAlgo API
    /// - Outputs can be non-zero
    /// - Inputs can be of different lengths
    fn test_edges<B: Algo<Add>>() {
        let a = &[1u8, 1];
        let b = &[1];

        assert_eq!(B::wrapping::<[_; _], _, _>(a, b), [2]);
        assert_eq!(B::wrapping::<[_; _], _, _>(a, b), [2, 1]);
        assert_eq!(B::wrapping::<[_; _], _, _>(b, a), [2]);
        assert_eq!(B::wrapping::<[_; _], _, _>(b, a), [2, 1]);
    }

    #[test]
    fn test_element() {
        #[cfg(feature = "alloc")]
        test_long::<Element>();
        test_wrapping::<Element>();
        test_saturating::<Element>();
        test_edges::<Element>();
    }

    #[test]
    fn test_bitwise() {
        #[cfg(feature = "alloc")]
        test_long::<Bitwise>();
        test_wrapping::<Bitwise>();
        test_saturating::<Bitwise>();
        test_edges::<Bitwise>();
    }
}
