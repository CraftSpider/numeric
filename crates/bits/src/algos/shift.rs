use crate::algos::{Algo, AssignAlgo, Bitwise, Element, Shl, Shr};
use crate::bit_slice::{BitLike, BitOwned, BitSlice};
use numeric_traits::class::Bounded;
use numeric_traits::identity::Zero;

impl Algo<Shl> for Bitwise {
    fn overflowing<O, L, R>(left: &L, right: usize) -> (O, bool)
    where
        O: BitOwned<Bit = L::Bit>,
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        let arr_shift = (right / L::Bit::BIT_LEN) + 1;
        let mut out = O::zeroed(left.len() + arr_shift);
        let overflow = <Self as Algo<Shl>>::overflowing_into::<_, R, _>(left, right, &mut out);
        (out, overflow)
    }

    fn overflowing_into<L, R, O>(left: &L, right: usize, out: &mut O) -> bool
    where
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice,
        O: BitOwned<Bit = L::Bit>,
    {
        let bit_len = left.bit_len();
        left.iter_bits().enumerate().for_each(|(idx, new)| {
            if new || idx + right < bit_len {
                out.set_bit_ignore(idx + right, new);
            }
        });
        out.shrink();
        right > bit_len
    }
}

impl Algo<Shr> for Bitwise {
    fn overflowing<O, L, R>(left: &L, right: usize) -> (O, bool)
    where
        O: BitOwned<Bit = L::Bit>,
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        let mut out = O::zeroed(left.len());
        let overflow = <Self as Algo<Shr>>::overflowing_into::<_, R, _>(left, right, &mut out);
        (out, overflow)
    }

    fn overflowing_into<L, R, O>(left: &L, right: usize, out: &mut O) -> bool
    where
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice,
        O: BitOwned<Bit = L::Bit>,
    {
        let bit_len = left.bit_len();
        for idx in (0..=bit_len).rev() {
            let new = left.get_bit(idx).unwrap_or(false);
            if let Some(idx) = idx.checked_sub(right) {
                out.set_bit_ignore(idx, new);
            }
        }
        out.shrink();
        right > bit_len
    }
}

impl Algo<Shl> for Element {
    fn overflowing<O, L, R>(left: &L, right: usize) -> (O, bool)
    where
        O: BitOwned<Bit = L::Bit>,
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        let arr_shift = (right / L::Bit::BIT_LEN) + 1;
        let mut out = O::zeroed(left.len() + arr_shift);
        let overflow = <Self as Algo<Shl>>::overflowing_into::<_, R, _>(left, right, &mut out);
        (out, overflow)
    }

    fn overflowing_into<L, R, O>(left: &L, right: usize, out: &mut O) -> bool
    where
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
        O: BitOwned<Bit = L::Bit>,
    {
        let arr_shift = (right / L::Bit::BIT_LEN) + 1;
        let elem_shift = right % L::Bit::BIT_LEN;
        let inverse_elem_shift = (L::Bit::BIT_LEN - elem_shift) % L::Bit::BIT_LEN;
        let elem_mask: L::Bit = !(L::Bit::max_value() << elem_shift);
        let zero = L::Bit::zero();

        left.iter().enumerate().rev().for_each(|(idx, val)| {
            let high = val >> inverse_elem_shift;
            let low = val << elem_shift;

            let high = (out.get(idx + arr_shift).unwrap_or(zero) & !elem_mask) | (high & elem_mask);

            out.set_ignore(idx + arr_shift, high);

            // We don't need to consider the existing value of the output. `low` always goes
            // into it before `high` since we're iterating backwards.
            let low = low & !elem_mask;

            out.set_ignore(idx + arr_shift - 1, low);
        });

        out.shrink();
        right > left.bit_len()
    }
}

impl AssignAlgo<Shl> for Element {
    fn overflowing<O, L, R>(left: &mut L, right: usize) -> bool
    where
        O: BitOwned<Bit = L::Bit>,
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        let arr_shift = (right / L::Bit::BIT_LEN) + 1;
        let elem_shift = right % L::Bit::BIT_LEN;
        let inverse_elem_shift = (L::Bit::BIT_LEN - elem_shift) % L::Bit::BIT_LEN;
        let elem_mask: L::Bit = !(L::Bit::max_value() << elem_shift);
        let zero = L::Bit::zero();

        (0..left.len()).rev().for_each(|idx| {
            // SAFETY: Iterating up to len - will never overrun
            let val = unsafe { left.get(idx).unwrap_unchecked() };
            let high = val >> inverse_elem_shift;
            let low = val << elem_shift;

            let high =
                (left.get(idx + arr_shift).unwrap_or(zero) & !elem_mask) | (high & elem_mask);

            left.set_ignore(idx + arr_shift, high);

            let low = low & !elem_mask;

            left.set_ignore(idx + arr_shift - 1, low);
        });
        left.iter_mut().take(arr_shift - 1).for_each(|l| *l = zero);

        right > left.bit_len()
    }
}

impl Algo<Shr> for Element {
    fn overflowing<O, L, R>(left: &L, right: usize) -> (O, bool)
    where
        O: BitOwned<Bit = L::Bit>,
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        let mut out = O::zeroed(left.len());
        let overflow = <Self as Algo<Shr>>::overflowing_into::<_, R, _>(left, right, &mut out);
        (out, overflow)
    }

    fn overflowing_into<L, R, O>(left: &L, right: usize, out: &mut O) -> bool
    where
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
        O: BitOwned<Bit = L::Bit>,
    {
        let arr_shift = (right / L::Bit::BIT_LEN) + 1;
        let elem_shift = right % L::Bit::BIT_LEN;
        let inverse_elem_shift = (L::Bit::BIT_LEN - elem_shift) % L::Bit::BIT_LEN;
        let elem_mask: L::Bit = !(L::Bit::max_value() >> elem_shift);
        let zero = L::Bit::zero();

        left.iter().enumerate().for_each(|(idx, val)| {
            let high = val >> elem_shift;
            let low = val << inverse_elem_shift;

            if let Some(idx) = usize::checked_sub(idx, arr_shift) {
                let low = (out.get(idx).unwrap_or(zero) & !elem_mask) | (low & elem_mask);

                out.set_ignore(idx, low);
            }

            if let Some(idx) = usize::checked_sub(idx + 1, arr_shift) {
                let high = high & !elem_mask;

                out.set_ignore(idx, high);
            }
        });

        out.shrink();
        right > left.bit_len()
    }
}

impl AssignAlgo<Shr> for Element {
    fn overflowing<O, L, R>(left: &mut L, right: usize) -> bool
    where
        O: BitOwned<Bit = L::Bit>,
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        let arr_shift = (right / L::Bit::BIT_LEN) + 1;
        let elem_shift = right % L::Bit::BIT_LEN;
        let inverse_elem_shift = (L::Bit::BIT_LEN - elem_shift) % L::Bit::BIT_LEN;
        let elem_mask: L::Bit = !(L::Bit::max_value() >> elem_shift);
        let zero = L::Bit::zero();

        // dbg!(arr_shift, elem_shift);

        (0..left.len()).for_each(|idx| {
            // SAFETY: Iterating up to len - will never overrun
            let val = unsafe { left.get(idx).unwrap_unchecked() };
            let high = val >> elem_shift;
            let low = val << inverse_elem_shift;

            if let Some(idx) = usize::checked_sub(idx, arr_shift) {
                let low = (left.get(idx).unwrap_or(zero) & !elem_mask) | (low & elem_mask);

                left.set_ignore(idx, low);
            }

            if let Some(idx) = usize::checked_sub(idx + 1, arr_shift) {
                let high = high & !elem_mask;

                left.set_ignore(idx, high);
            }
        });
        let empty = left.len() - arr_shift + 1;
        left.iter_mut().skip(empty).for_each(|l| *l = zero);

        right > left.bit_len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algos::{Bitwise, Element};

    #[cfg(feature = "alloc")]
    fn test_shl<B: Algo<Shl>>() {
        use alloc::vec::Vec;
        let slice: &[u16] = &[0b1010_1010_1010_1010, 0b1010_1010_1010_1010];
        assert_eq!(
            B::wrapping::<Vec<_>, _, [_]>(slice, 1),
            &[0b0101_0101_0101_0100, 0b0101_0101_0101_0101, 0b1]
        );

        let slice: &[u8] = &[0b1111_1111];
        assert_eq!(B::wrapping::<Vec<_>, _, [_]>(slice, 8), &[0b0, 0b1111_1111]);

        assert_eq!(B::wrapping::<Vec<_>, _, [_]>(&[0b0000_0000u8], 1), &[0]);
        assert_eq!(B::wrapping::<Vec<_>, _, [_]>(&[0b01u8], 1), &[0b10]);
        assert_eq!(B::wrapping::<Vec<_>, _, [_]>(&[0b0101u8], 1), &[0b1010]);

        let slice = &[0b1010_1010_1010_1010u16, 0b1010_1010_1010_1010];
        let res = B::wrapping::<Vec<_>, _, [_]>(slice, 1);
        assert_eq!(res, &[0b0101_0101_0101_0100, 0b0101_0101_0101_0101, 0b1]);
        assert_eq!(B::wrapping::<Vec<_>, _, [_]>(&[0b1u8], 8), &[0b0, 0b1])
    }

    fn test_shl_wrapping<B: Algo<Shl>>() {
        let data = [0u8];
        assert_eq!(B::wrapping::<[_; _], _, [_]>(&data, 1), [0]);
        let data = [0b01u8];
        assert_eq!(B::wrapping::<[_; _], _, [_]>(&data, 1), [0b10]);
        let data = [0b0101u8];
        assert_eq!(B::wrapping::<[_; _], _, [_]>(&data, 1), [0b1010]);

        let data = [0b1010_1010_1010_1010u16, 0b1010_1010_1010_1010];
        assert_eq!(
            B::wrapping::<[_; _], _, [_]>(&data, 1),
            [0b0101_0101_0101_0100, 0b0101_0101_0101_0101],
        );
        let data = [0b1000_0000u8, 0b1000_0000];
        assert_eq!(B::wrapping::<[_; _], _, [_]>(&data, 1), [0b0, 0b1]);
        let data = [0b1u8, 0b0];
        assert_eq!(B::wrapping::<[_; _], _, [_]>(&data, 8), [0b0, 0b1])
    }

    #[cfg(feature = "alloc")]
    fn test_shr<B: Algo<Shr>>() {
        use alloc::vec::Vec;
        let slice: &[u16] = &[0b1010_1010_1010_1010, 0b1010_1010_1010_1010];
        assert_eq!(
            B::wrapping::<Vec<_>, _, [_]>(slice, 1),
            [0b0101_0101_0101_0101, 0b0101_0101_0101_0101]
        );

        let slice: &[u8] = &[0b0, 0b1111_1111];
        assert_eq!(B::wrapping::<Vec<_>, _, [_]>(slice, 8), [0b1111_1111]);
    }

    fn test_shr_wrapping<B: Algo<Shr>>() {
        let data = [0u8];
        assert_eq!(B::wrapping::<[_; _], _, [_]>(&data, 1), [0]);
        let data = [0b10u8];
        assert_eq!(B::wrapping::<[_; _], _, [_]>(&data, 1), [0b01]);
        let data = [0b1010u8];
        assert_eq!(B::wrapping::<[_; _], _, [_]>(&data, 1), [0b0101]);

        let data = [0b1010_1010_1010_1010u16, 0b1010_1010_1010_1010];
        assert_eq!(
            B::wrapping::<[_; _], _, [_]>(&data, 1),
            [0b0101_0101_0101_0101, 0b0101_0101_0101_0101],
        );
        let data = [0b0000_0001u8, 0b0000_0001];
        assert_eq!(B::wrapping::<[_; _], _, [_]>(&data, 1), [0b1000_0000, 0b0]);
        let data = [0b0u8, 0b1];
        assert_eq!(B::wrapping::<[_; _], _, [_]>(&data, 8), [0b1, 0b0])
    }

    #[test]
    fn test_bitwise() {
        #[cfg(feature = "alloc")]
        test_shl::<Bitwise>();
        #[cfg(feature = "alloc")]
        test_shr::<Bitwise>();

        test_shl_wrapping::<Bitwise>();
        test_shr_wrapping::<Bitwise>();
    }

    #[test]
    fn test_element() {
        #[cfg(feature = "alloc")]
        test_shl::<Element>();
        #[cfg(feature = "alloc")]
        test_shr::<Element>();

        test_shl_wrapping::<Element>();
        test_shr_wrapping::<Element>();
    }
}
