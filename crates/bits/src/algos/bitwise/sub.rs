use crate::algos::ElementNot;
use crate::bit_slice::BitSliceExt;
#[cfg(feature = "std")]
use alloc::{vec, vec::Vec};
use numeric_traits::identity::Zero;

pub trait BitwiseSub: BitSliceExt {
    #[cfg(feature = "std")]
    /// Subtract two slices, implemented as a bitwise sub-and-borrow
    fn sub<T>(left: &Self, right: &T) -> (Vec<Self::Bit>, bool)
    where
        T: ?Sized + BitSliceExt<Bit = Self::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let bit_len = usize::max(left.bit_len(), right.bit_len());
        let mut out = vec![Self::Bit::zero(); len];

        let mut carry = false;
        for idx in 0..bit_len {
            let l = left.get_bit_opt(idx).unwrap_or(false);
            let r = right.get_bit_opt(idx).unwrap_or(false);

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
            ElementNot::not(&mut out);
        }

        (out, carry)
    }
}

impl<T> BitwiseSub for T where T: ?Sized + BitSliceExt {}
