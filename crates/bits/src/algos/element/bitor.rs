use numeric_traits::identity::Zero;
use crate::bit_slice::BitSliceExt;
#[cfg(feature = "std")]
use alloc::{vec, vec::Vec};

pub trait ElementBitor: BitSliceExt {
    #[cfg(feature = "std")]
    fn bitor<T>(left: &Self, right: &T) -> Vec<Self::Bit>
    where
        T: ?Sized + BitSliceExt<Bit = Self::Bit>,
    {
        let len = usize::max(left.len(), right.len());
        let zero = Self::Bit::zero();
        let mut out = vec![zero; len];

        for idx in 0..len {
            let l = left.get_opt(idx).unwrap_or(zero);
            let r = right.get_opt(idx).unwrap_or(zero);

            out.set(idx, l | r);
        }

        out
    }
}

impl<T> ElementBitor for T
where
    T: ?Sized + BitSliceExt,
{}
