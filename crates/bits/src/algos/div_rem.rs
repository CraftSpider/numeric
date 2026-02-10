use crate::bit_slice::BitSliceExt;

mod impls;

pub trait DivRemAlgo {
    #[cfg(feature = "std")]
    fn long<L, R>(left: &L, right: &R) -> (alloc::vec::Vec<L::Bit>, alloc::vec::Vec<L::Bit>)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>;

    fn overflowing<'a, L, R>(
        left: &L,
        right: &R,
        quotient: &'a mut [L::Bit],
        remainder: &'a mut [L::Bit],
    ) -> (&'a [L::Bit], &'a [L::Bit], bool)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>;

    fn wrapping<'a, L, R>(
        left: &L,
        right: &R,
        quotient: &'a mut [L::Bit],
        remainder: &'a mut [L::Bit],
    ) -> (&'a [L::Bit], &'a [L::Bit])
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let (q, r, _) = Self::overflowing(left, right, quotient, remainder);
        (q, r)
    }

    fn checked<'a, L, R>(
        left: &L,
        right: &R,
        quotient: &'a mut [L::Bit],
        remainder: &'a mut [L::Bit],
    ) -> Option<(&'a [L::Bit], &'a [L::Bit])>
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let (q, r, overflow) = Self::overflowing(left, right, quotient, remainder);
        if overflow {
            None
        } else {
            Some((q, r))
        }
    }

    fn saturating<'a, L, R>(
        left: &L,
        right: &R,
        quotient: &'a mut [L::Bit],
        remainder: &'a mut [L::Bit],
    ) -> (&'a [L::Bit], &'a [L::Bit])
    where
        L: BitSliceExt,
        R: BitSliceExt<Bit = L::Bit>,
    {
        let (q, r, overflow) = Self::overflowing(left, right, quotient, remainder);
        if overflow {
            todo!()
        } else {
            // SAFETY: Polonius case
            unsafe { core::mem::transmute::<(&[_], &[_]), (&[_], &[_])>((q, r)) }
        }
    }
}

pub trait AssignDivRemAlgo {
    fn div_overflowing<L, R>(left: &mut L, right: &R, remainder: &mut [L::Bit]) -> bool
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>;

    fn div_wrapping<L, R>(left: &mut L, right: &R, remainder: &mut [L::Bit])
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        Self::div_overflowing(left, right, remainder);
    }

    fn div_checked<L, R>(left: &mut L, right: &R, remainder: &mut [L::Bit]) -> Option<()>
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        if Self::div_overflowing(left, right, remainder) {
            None
        } else {
            Some(())
        }
    }

    fn div_saturating<L, R>(left: &mut L, right: &R, remainder: &mut [L::Bit])
    where
        L: BitSliceExt,
        R: BitSliceExt<Bit = L::Bit>,
    {
        let overflow = Self::div_overflowing(left, right, remainder);
        if overflow {
            todo!()
        }
    }

    fn rem_overflowing<L, R>(left: &mut L, right: &R, quotient: &mut [L::Bit]) -> bool
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>;

    fn rem_wrapping<L, R>(left: &mut L, right: &R, quotient: &mut [L::Bit])
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        Self::rem_overflowing(left, right, quotient);
    }

    fn rem_checked<L, R>(left: &mut L, right: &R, quotient: &mut [L::Bit]) -> Option<()>
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        if Self::rem_overflowing(left, right, quotient) {
            None
        } else {
            Some(())
        }
    }

    fn rem_saturating<L, R>(left: &mut L, right: &R, quotient: &mut [L::Bit])
    where
        L: BitSliceExt,
        R: BitSliceExt<Bit = L::Bit>,
    {
        let overflow = Self::rem_overflowing(left, right, quotient);
        if overflow {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algos::Bitwise;

    fn test_div_long<B: DivRemAlgo>() {
        let slice1: &[u8] = &[0b10];
        let slice2: &[u8] = &[0b01];

        assert_eq!(B::long(slice1, slice2).0, &[0b10]);

        let slice3: &[u8] = &[0b10];
        let slice4: &[u8] = &[0b10];

        assert_eq!(B::long(slice3, slice4).0, &[0b01]);

        let slice5: &[u8] = &[0b00000000, 0b1];
        let slice6: &[u8] = &[0b00000010];

        assert_eq!(B::long(slice5, slice6).0, &[0b10000000, 0b0]);

        let slice7: &[u8] = &[0b0, 0b0, 0b0, 0b1];
        let slice8: &[u8] = &[0b10];

        assert_eq!(B::long(slice7, slice8).0, &[0b0, 0b0, 0b10000000, 0b0]);
    }

    fn test_rem_long<B: DivRemAlgo>() {
        for i in 0..4 {
            let slice1: &[u8] = &[i];
            let slice2 = &[0b10];

            assert_eq!(B::long(slice1, slice2).1, &[i % 2]);
        }

        for i in 0..6 {
            let slice3: &[u8] = &[i];
            let slice4 = &[0b11];

            assert_eq!(B::long(slice3, slice4).1, &[i % 3]);
        }

        let slice5: &[u8] = &[0b00000001, 0b111];
        let slice6 = &[0b00000010];

        assert_eq!(B::long(slice5, slice6).1, &[0b01, 0b0]);
    }

    fn test_div_wrapping<B: DivRemAlgo>() {
        let data = &[0b10u8];
        let slice2: &[u8] = &[0b01];

        assert_eq!(B::wrapping(data, slice2, &mut [0], &mut [0]).0, &[0b10]);

        let data = &[0b10u8];
        let slice4: &[u8] = &[0b10];

        assert_eq!(B::wrapping(data, slice4, &mut [0], &mut [0]).0, &[0b01]);

        let data = &[0b00000000u8, 0b1];
        let slice6: &[u8] = &[0b00000010];

        assert_eq!(
            B::wrapping(data, slice6, &mut [0; 2], &mut [0; 2]).0,
            &[0b10000000, 0b0]
        );

        let data = &[0b0u8, 0b0, 0b0, 0b1];
        let slice8: &[u8] = &[0b10];

        assert_eq!(
            B::wrapping(data, slice8, &mut [0; 4], &mut [0; 4]).0,
            &[0b0, 0b0, 0b10000000, 0b0]
        );
    }

    #[test]
    fn test_bitwise() {
        test_div_long::<Bitwise>();
        test_rem_long::<Bitwise>();

        test_div_wrapping::<Bitwise>();
    }
}
