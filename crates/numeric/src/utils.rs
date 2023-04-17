use numeric_traits::class::{Unsigned, Bounded, Integral};
use numeric_traits::cast::FromChecked;

pub fn shrink_slice<T: Integral + Copy>(val: &[T]) -> &[T] {
    let mut idx = 0;
    for i in (0..val.len()).rev() {
        // This ensures no bounds checks ever get generated
        // SAFETY: We iterate up to length
        if unsafe { *val.get_unchecked(i) } != T::zero() {
            idx = i;
            break;
        }
    }
    &val[..=idx]
}

pub fn shrink_vec<T: Integral + Copy>(mut val: Vec<T>) -> Vec<T> {
    while val.len() > 1 && val.last() == Some(&T::zero()) {
        val.pop();
    }
    val
}

// Length 16 chosen as it's the longest possible result for u8 -> u128
pub fn int_to_arr<T, U>(val: T) -> [U; 16]
where
    T: Integral + FromChecked<U> + Bounded + Copy + Unsigned,
    U: Integral + FromChecked<T> + Bounded + Copy + Unsigned,
{
    let mut out = [U::zero(); 16];

    match U::from_checked(val) {
        Some(u) => out[0] = u,
        None => {
            let max: T = T::from_checked(U::max_value())
                .expect("Our value didn't fit in a T - must be too big") + T::one();

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
                        *item = U::from_checked(rem).expect("Modulo U::max_value() + 1 - Will always be >= U::max_value()");
                    }
                }
            }
        }
    }

    out
}

pub fn arr_to_int<T: Integral + Copy, U: Integral + FromChecked<T> + Copy>(arr: &[T]) -> Option<U> {
    let mut out = U::zero();
    for (idx, &i) in arr.iter().enumerate() {
        let t = U::from_checked(i)?;
        out = out + (t << (idx * (usize::BITS as usize)));
    }
    Some(out)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shrink_slice() {
        assert_eq!(shrink_slice(&[0]), &[0]);
        assert_eq!(shrink_slice(&[0, 0]), &[0]);
        assert_eq!(shrink_slice(&[0, 0, 0]), &[0]);
        assert_eq!(shrink_slice(&[0, 1]), &[0, 1]);
        assert_eq!(shrink_slice(&[0, 1, 1]), &[0, 1, 1]);

        assert_eq!(shrink_slice(&[1]), &[1]);
        assert_eq!(shrink_slice(&[1, 1]), &[1, 1]);
        assert_eq!(shrink_slice(&[1, 1, 1]), &[1, 1, 1]);
        assert_eq!(shrink_slice(&[1, 0]), &[1]);
        assert_eq!(shrink_slice(&[1, 0, 0]), &[1]);
    }

    #[test]
    fn test_int_arr() {
        assert_eq!(&int_to_arr::<u16, usize>(0), &[0; 16]);
        assert_eq!(&int_to_arr::<u16, usize>(1), &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(&int_to_arr::<u16, usize>(u16::MAX), &[u16::MAX as usize, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        assert_eq!(&int_to_arr::<u128, usize>(0), &[0; 16]);
        assert_eq!(&int_to_arr::<u128, usize>(1), &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(&int_to_arr::<u128, usize>(usize::MAX as u128 + 1), &[0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(&int_to_arr::<u128, usize>(u128::MAX), &[usize::MAX, usize::MAX, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        assert_eq!(
            &int_to_arr::<u128, u8>(0x0102030405060708090A0B0C0D0E0F00),
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
    }
}
