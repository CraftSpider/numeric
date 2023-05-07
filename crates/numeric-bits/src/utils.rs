use std::ops::Deref;
use numeric_traits::class::{Unsigned, Bounded, Integral};
use numeric_traits::cast::{FromAll, FromChecked};
use numeric_traits::ops::checked::CheckedShl;
use numeric_traits::ops::core::NumAssignOps;

pub trait IntSlice<T>: Deref<Target = [T]> {
    fn shrink(self) -> Self;
}

impl<T: Integral + Copy> IntSlice<T> for &[T] {
    fn shrink(self) -> Self {
        // TODO: For some reason the below impl is slower, despite having no panics
        //       and less asm. Branch prediction is the best bet why so far.
        // while let Some((last, rest @ [_, ..])) = val.split_last() {
        //     if *last != T::zero() {
        //         break
        //     }
        //     val = rest;
        // }
        // val

        let mut idx = 0;
        for i in (0..self.len()).rev() {
            // This ensures no bounds checks ever get generated
            // SAFETY: We iterate up to length
            if unsafe { *self.get_unchecked(i) } != T::zero() {
                idx = i;
                break;
            }
        }
        &self[..=idx]
    }
}

impl<T: Integral + Copy> IntSlice<T> for Vec<T> {
    fn shrink(mut self) -> Self {
        while self.len() > 1 && self.last() == Some(&T::zero()) {
            self.pop();
        }
        self
    }
}

// Length 16 chosen as it's the longest possible result for u8 -> u128
pub fn int_to_arr<T, U, const N: usize>(val: T) -> [U; N]
where
    T: Integral + FromAll<U> + Bounded + Copy + Unsigned,
    U: Integral + FromAll<T> + Bounded + Copy + Unsigned,
{
    debug_assert!(N != 0);

    let mut out = [U::zero(); N];

    match U::from_checked(val) {
        Some(u) => out[0] = u,
        None => {
            // This is fine to truncate - if our value didn't fit in a `T`, we should
            // have succeeded our earlier check
            let max: T = T::truncate(U::max_value()) + T::one();

            let mut left = val;
            for item in &mut out {
                match U::from_checked(left) {
                    Some(u) => {
                        *item = u;
                        break;
                    }
                    None => {
                        let rem: T = left % max;
                        left = left / max;
                        // Modulo U::max_value() + 1 - Will always be <= U::max_value();
                        *item = U::truncate(rem);
                    }
                }
            }
        }
    }

    out
}

pub fn arr_to_int<T: Integral + Copy, U: Integral + CheckedShl<usize, Output = U> + NumAssignOps + FromChecked<T> + Copy>(arr: &[T]) -> Option<U> {
    let bit_len = core::mem::size_of::<T>() * 8;
    let mut out = U::zero();
    for (idx, &i) in arr.iter().enumerate() {
        let t = U::from_checked(i)?;
        out += t.checked_shl(idx * bit_len)?;
    }
    Some(out)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shrink_slice() {
        assert_eq!(IntSlice::shrink(&[0] as &[_]), &[0]);
        assert_eq!(IntSlice::shrink(&[0, 0] as &[_]), &[0]);
        assert_eq!(IntSlice::shrink(&[0, 0, 0] as &[_]), &[0]);
        assert_eq!(IntSlice::shrink(&[0, 1] as &[_]), &[0, 1]);
        assert_eq!(IntSlice::shrink(&[0, 1, 1] as &[_]), &[0, 1, 1]);

        assert_eq!(IntSlice::shrink(&[1] as &[_]), &[1]);
        assert_eq!(IntSlice::shrink(&[1, 1] as &[_]), &[1, 1]);
        assert_eq!(IntSlice::shrink(&[1, 1, 1] as &[_]), &[1, 1, 1]);
        assert_eq!(IntSlice::shrink(&[1, 0] as &[_]), &[1]);
        assert_eq!(IntSlice::shrink(&[1, 0, 0] as &[_]), &[1]);

        assert_eq!(IntSlice::shrink(&[1, 0, 1] as &[_]), &[1, 0, 1]);
    }

    #[test]
    fn test_int_arr() {
        assert_eq!(&int_to_arr::<u16, usize, 1>(0), &[0]);
        assert_eq!(&int_to_arr::<u16, usize, 1>(1), &[1]);
        assert_eq!(&int_to_arr::<u16, usize, 1>(u16::MAX), &[u16::MAX as usize]);

        assert_eq!(&int_to_arr::<u128, usize, 2>(0), &[0; 2]);
        assert_eq!(&int_to_arr::<u128, usize, 2>(1), &[1, 0]);
        assert_eq!(&int_to_arr::<u128, usize, 2>(usize::MAX as u128 + 1), &[0, 1]);
        assert_eq!(&int_to_arr::<u128, usize, 2>(u128::MAX), &[usize::MAX, usize::MAX]);

        assert_eq!(
            &int_to_arr::<u128, u8, 16>(0x0102030405060708090A0B0C0D0E0F00),
            &[0, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1],
        );
    }

    #[test]
    fn test_arr_int() {
        assert_eq!(arr_to_int::<usize, u8>(&[0]), Some(0));
        assert_eq!(arr_to_int::<usize, u8>(&[1]), Some(1));
        assert_eq!(arr_to_int::<usize, u8>(&[usize::MAX]), None);

        assert_eq!(arr_to_int::<usize, u128>(&[0]), Some(0));
        assert_eq!(arr_to_int::<usize, u128>(&[1]), Some(1));
        assert_eq!(arr_to_int::<usize, u128>(&[usize::MAX]), Some(usize::MAX as u128));

        assert_eq!(arr_to_int::<u8, usize>(&[0, 1, 2, 3]), Some(0x03020100));
        assert_eq!(arr_to_int::<u8, usize>(&[u8::MAX]), Some(255));
    }
}
