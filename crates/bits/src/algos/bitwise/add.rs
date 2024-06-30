use core::hint::unreachable_unchecked;
#[cfg(feature = "std")]
use alloc::{vec, vec::Vec};
use numeric_traits::identity::Zero;
use crate::bit_slice::{BitSliceExt, BitVecExt};

pub trait BitwiseAdd: BitSliceExt {
    #[cfg(feature = "std")]
    /// Add two slices, implemented as a bitwise add-and-carry
    fn add<T>(left: &Self, right: &T) -> Vec<Self::Bit>
    where
        T: ?Sized + BitSliceExt<Bit = Self::Bit>
    {
        let len = usize::max(left.len(), right.len());
        let bit_len = usize::max(left.bit_len(), right.bit_len());
        let mut out = vec![Self::Bit::zero(); len];

        let mut carry = false;
        for idx in 0..=bit_len {
            let l = u8::from(left.get_bit_opt(idx).unwrap_or(false));
            let r = u8::from(right.get_bit_opt(idx).unwrap_or(false));

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

            if new {
                out.set_bit_push(idx, new);
            }
        }

        out
    }

    /// Add two slices, implemented as a bitwise add-and-carry, storing the result in the lefthand
    /// slice.
    fn add_in_place<'a, T>(left: &'a mut Self, right: &T) -> &'a mut Self
    where
        T: BitSliceExt<Bit = Self::Bit>,
    {
        let bit_len = usize::max(left.bit_len(), right.bit_len());

        let mut carry = false;
        for idx in 0..=bit_len {
            let l = u8::from(left.get_bit(idx));
            let r = u8::from(right.get_bit(idx));

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

            left.set_bit(idx, new);
        }

        left
    }
}

impl<T> BitwiseAdd for T
where
    T: ?Sized + BitSliceExt,
{}
