use crate::algos::{Algo, AssignAlgo, Bitwise, Element, Mul};
use crate::bit_slice::{BitOwned, BitSlice};
use numeric_traits::identity::{One, Zero};
use numeric_traits::ops::overflowing::OverflowingAdd;
use numeric_traits::ops::widening::WideningMul;

impl Algo<Mul> for Element {
    fn overflowing<O, L, R>(left: &L, right: &R) -> (O, bool)
    where
        O: BitOwned<Bit = L::Bit>,
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let mut out = O::zeroed(len * 2);
        let overflow = <Self as Algo<Mul>>::overflowing_into::<L, R, O>(left, right, &mut out);
        (out, overflow)
    }

    fn overflowing_into<L, R, O>(left: &L, right: &R, out: &mut O) -> bool
    where
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
        O: BitOwned<Bit = L::Bit>,
    {
        fn inner<
            L: ?Sized + BitSlice,
            R: ?Sized + BitSlice<Bit = L::Bit>,
            O: BitOwned<Bit = L::Bit>,
        >(
            long: &L,
            short: &R,
            out: &mut O,
        ) -> bool {
            let zero = L::Bit::zero();

            let mut overflow = false;
            for (idx, l) in long.iter().enumerate() {
                // From the top to bottom, add N shifted copies of M. This can be done by taking each
                // element of the left and doing a widening mul, carrying the upper, and repeating
                let mut new_overflow = false;
                let mut carry = zero;

                out.set_ignore(idx, zero);

                for (offset, r) in short.iter().enumerate() {
                    let (low, high) = L::Bit::widening_mul(l, r, carry);
                    carry = high;
                    if add_item(out, idx + offset, low) {
                        new_overflow = true;
                    }
                }

                if carry != zero && add_item(out, idx + short.len(), carry) {
                    new_overflow = true;
                }

                overflow |= new_overflow;
            }

            overflow
        }

        let overflow = if left.len() > right.len() {
            inner(left, right, out)
        } else {
            inner(right, left, out)
        };
        out.shrink();
        overflow
    }
}

impl AssignAlgo<Mul> for Element {
    fn overflowing<O, L, R>(left: &mut L, right: &R) -> bool
    where
        O: BitOwned,
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        let zero = L::Bit::zero();

        let mut overflow = false;
        for idx in (0..left.len()).rev() {
            // From the top to bottom, add N shifted copies of M. This can be done by taking each
            // element of the left and doing a widening mul, carrying the upper, and repeating
            let mut new_overflow = false;
            let mut carry = zero;

            let l = left.get(idx).unwrap();
            left.set(idx, zero);

            for (offset, r) in right.iter().enumerate() {
                let (low, high) = L::Bit::widening_mul(l, r, carry);
                carry = high;
                if add_item(left, idx + offset, low) {
                    new_overflow = true;
                }
            }

            if carry != zero && add_item(left, idx + right.len(), carry) {
                new_overflow = true;
            }

            overflow |= new_overflow;
        }

        overflow
    }
}

impl Algo<Mul> for Bitwise {
    fn overflowing<O, L, R>(left: &L, right: &R) -> (O, bool)
    where
        O: BitOwned<Bit = L::Bit>,
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let mut out = O::zeroed(len * 2);
        let overflow = <Self as Algo<Mul>>::overflowing_into::<L, R, O>(left, right, &mut out);
        (out, overflow)
    }

    fn overflowing_into<L, R, O>(left: &L, right: &R, out: &mut O) -> bool
    where
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
        O: BitOwned<Bit = L::Bit>,
    {
        out.iter_mut().for_each(|v| *v = L::Bit::zero());
        let mut overflow = false;
        for idx in 0..right.bit_len() {
            let r = right.get_bit(idx).unwrap_or(false);

            if r {
                let mut new_overflow = false;
                for (offset, l) in left.iter_bits().enumerate() {
                    if l && add_bit(out, idx + offset) {
                        new_overflow = true;
                    }
                }
                overflow |= new_overflow;
            }
        }
        out.shrink();
        overflow
    }
}

impl AssignAlgo<Mul> for Bitwise {
    fn overflowing<O, L, R>(left: &mut L, right: &R) -> bool
    where
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
        O: BitOwned<Bit = L::Bit>,
    {
        let mut overflow = false;
        for idx in (0..left.bit_len()).rev() {
            let l = left.get_bit(idx).unwrap_or(false);
            left.set_bit(idx, false);

            if l {
                let mut new_overflow = false;
                for (offset, r) in right.iter_bits().enumerate() {
                    if r && add_bit(left, idx + offset) {
                        new_overflow = true;
                    }
                }
                overflow |= new_overflow;
            }
        }
        overflow
    }
}

fn add_bit<B: ?Sized + BitSlice>(slice: &mut B, mut idx: usize) -> bool {
    let mut carry = false;
    while let Some(val) = slice.get_bit(idx) {
        slice.set_bit(idx, true);
        if val {
            carry = true;
        }
        idx += 1;

        if !carry {
            break;
        }
    }
    carry
}

fn add_item<B: ?Sized + BitSlice>(slice: &mut B, mut idx: usize, mut val: B::Bit) -> bool {
    let mut carry = false;

    while let Some(loc) = slice.get_mut(idx) {
        let (new, new_carry) = loc.overflowing_add(val);
        carry = new_carry;
        *loc = new;
        idx += 1;

        if !carry {
            break;
        } else {
            val = B::Bit::one();
        }
    }

    carry
}

#[cfg(test)]
mod tests {
    use crate::algos::{Algo, AssignAlgo, Bitwise, Element, Mul};

    #[cfg(feature = "alloc")]
    fn test_long<B: Algo<Mul>>() {
        use alloc::vec::Vec;
        let slice1: &[u8] = &[0b0000_0000];
        let slice2 = &[0b0000_0001];

        assert_eq!(B::wrapping::<Vec<_>, _, _>(slice1, slice2), &[0b0]);

        let slice3: &[u8] = &[0b0000_0001];
        let slice4 = &[0b0000_0001];

        assert_eq!(B::wrapping::<Vec<_>, _, _>(slice3, slice4), &[0b1]);

        let slice5: &[u8] = &[0b0000_0001];
        let slice6 = &[0b0000_0010];

        assert_eq!(B::wrapping::<Vec<_>, _, _>(slice5, slice6), &[0b10]);

        let slice7: &[u8] = &[0b0000_0010];
        let slice8 = &[0b0000_0010];

        assert_eq!(B::wrapping::<Vec<_>, _, _>(slice7, slice8), &[0b100]);
    }

    fn test_wrapping<B: Algo<Mul>>() {
        let slice1: &[u8] = &[0b0000_0000];
        let slice2 = &[0b0000_0001];
        assert_eq!(B::wrapping::<[u8; 1], _, _>(slice1, slice2), [0b0]);

        let slice3: &[u8] = &[0b0000_0001];
        let slice4 = &[0b0000_0001];
        assert_eq!(B::wrapping::<[u8; 1], _, _>(slice3, slice4), [0b1]);

        let slice5: &[u8] = &[0b0000_0001];
        let slice6 = &[0b0000_0010];
        assert_eq!(B::wrapping::<[u8; 1], _, _>(slice5, slice6), [0b10]);

        let slice7: &[u8] = &[0b0000_0010];
        let slice8 = &[0b0000_0010];
        assert_eq!(B::wrapping::<[u8; 1], _, _>(slice7, slice8), [0b100]);
    }

    fn test_wrapping_assign<B: AssignAlgo<Mul>>() {
        let slice1: &mut [u8] = &mut [0b0000_0000];
        let slice2 = &[0b0000_0001];

        B::wrapping::<[u8; 0], _, _>(slice1, slice2);
        assert_eq!(slice1, &[0b0]);

        let slice3: &mut [u8] = &mut [0b0000_0001];
        let slice4 = &[0b0000_0001];

        B::wrapping::<[u8; 0], _, _>(slice3, slice4);
        assert_eq!(slice3, &[0b1]);

        let slice5: &mut [u8] = &mut [0b0000_0001];
        let slice6 = &[0b0000_0010];

        B::wrapping::<[u8; 0], _, _>(slice5, slice6);
        assert_eq!(slice5, &[0b10]);

        let slice7: &mut [u8] = &mut [0b0000_0010];
        let slice8 = &[0b0000_0010];

        B::wrapping::<[u8; 0], _, _>(slice7, slice8);
        assert_eq!(slice7, &[0b100]);
    }

    // fn test_high_assign<B: AssignMulAlgo>() {
    //     let slice1 = &mut [0b1000_0000u8];
    //     let slice2 = &[0b0000_0010];
    //
    //     B::high(slice1, slice2);
    //     assert_eq!(slice1, &[0b0000_0001]);
    //
    //     let slice3 = &mut [0b1111_1111u8];
    //     let slice4 = &[2];
    //     B::high(slice3, slice4);
    //     assert_eq!(slice3, &[0b1]);
    //
    //     let slice5 = &mut [0b1111_1111u8];
    //     let slice6 = &[0x10u8];
    //     B::high(slice5, slice6);
    //     assert_eq!(slice5, &[0b0000_1111]);
    //
    //     let slice7 = &mut [0b1000_0000u8, 0b0000_1000];
    //     let slice8 = &[0b0000_0000, 0b1000_0000];
    //     B::high(slice7, slice8);
    //     assert_eq!(slice7, &[0b0100_0000, 0b0000_0100]);
    //
    //     let l = &mut [0b1111_1111u8, 0b1111_1111];
    //     let r = &[0b1111_1111, 0b1111_1111];
    //     B::high(l, r);
    //     assert_eq!(l, &[0b11111110, 0b11111111]);
    // }

    /// Test some edge cases of the desired MulAlgo API
    /// - Inputs can be of different lengths
    fn test_edges<B: Algo<Mul>>() {
        let a = &[3u8, 3];
        let b = &[2];

        assert_eq!(B::wrapping::<[u8; 1], _, _>(a, b), [6]);
        assert_eq!(B::wrapping::<[u8; 2], _, _>(a, b), [6, 6]);
        assert_eq!(B::wrapping::<[u8; 1], _, _>(a, b), [6]);
        assert_eq!(B::wrapping::<[u8; 2], _, _>(a, b), [6, 6]);
    }

    #[test]
    fn test_element() {
        #[cfg(feature = "alloc")]
        test_long::<Element>();
        test_wrapping::<Element>();
        test_wrapping_assign::<Element>();
        test_edges::<Element>();
    }

    #[test]
    fn test_bitwise() {
        #[cfg(feature = "alloc")]
        test_long::<Bitwise>();
        test_wrapping::<Bitwise>();
        test_wrapping_assign::<Bitwise>();
        test_edges::<Bitwise>();
    }
}
