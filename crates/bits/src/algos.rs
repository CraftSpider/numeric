//! Various big integer algorithms.
//!
//! Algorithms that work on [bit-slice](crate::bit_slice) types to perform various mathematical
//! operations on arbitrary length, or even unbounded, integers. This provides a low-level base on
//! which to build various integer types not natively available in Rust.

#[allow(missing_docs)]
mod add;
#[allow(missing_docs)]
mod bits;
#[allow(missing_docs)]
mod cmp;
#[allow(missing_docs)]
mod div_rem;
#[allow(missing_docs)]
mod mul;
#[allow(missing_docs)]
mod shift;
#[allow(missing_docs)]
mod sub;

use crate::bit_slice::{BitLike, BitSliceExt};
use numeric_traits::class::Bounded;
use numeric_traits::identity::Zero;

pub trait BitOwned: BitSliceExt {
    fn zeroed(len: usize) -> Self;
    fn shrink(self) -> Self;
}

impl<T: BitLike, const N: usize> BitOwned for [T; N] {
    fn zeroed(_: usize) -> Self {
        [T::zero(); N]
    }

    fn shrink(self) -> Self {
        self
    }
}

#[cfg(feature = "alloc")]
impl<T: BitLike> BitOwned for alloc::vec::Vec<T> {
    fn zeroed(len: usize) -> Self {
        alloc::vec![T::zero(); len]
    }

    fn shrink(mut self) -> Self {
        let idx = self.iter().rposition(|val| val != T::zero()).unwrap_or(0);
        self.drain(idx + 1..);
        self
    }
}

pub trait AlgoTy {
    type Out<O: BitOwned>: BitOwned<Bit = O::Bit>;
}

pub struct Add;

impl AlgoTy for Add {
    type Out<O: BitOwned> = O;
}

pub trait Algo<A: AlgoTy> {
    const SATURATE_HIGH: bool;

    fn overflowing<L, R, O>(left: &L, right: &R) -> (A::Out<O>, bool)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
        O: BitOwned<Bit = L::Bit>;

    fn checked<L, R, O>(left: &L, right: &R) -> Option<A::Out<O>>
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
        O: BitOwned<Bit = L::Bit>,
    {
        let (val, overflow) = Self::overflowing(left, right);
        if overflow {
            None
        } else {
            Some(val)
        }
    }

    fn wrapping<L, R, O>(left: &L, right: &R) -> A::Out<O>
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
        O: BitOwned<Bit = L::Bit>,
    {
        Self::overflowing(left, right).0
    }

    fn saturating<L, R, O>(left: &L, right: &R) -> A::Out<O>
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
        O: BitOwned<Bit = L::Bit>,
    {
        let (mut val, overflow) = Self::overflowing(left, right);
        if overflow {
            if Self::SATURATE_HIGH {
                val.iter_mut().for_each(|v| *v = O::Bit::max_value());
            } else {
                val.iter_mut().for_each(|v| *v = O::Bit::zero());
            }
            val
        } else {
            val
        }
    }
}

pub use add::*;
pub use bits::*;
pub use cmp::*;
pub use div_rem::*;
pub use mul::*;
pub use shift::*;
pub use sub::*;

/// Simple bitwise implementations of algorithms. These implementations are generally inefficient,
/// but straightforward compared to alternative approaches.
pub struct Bitwise;

/// Element-wise implementations of algorithms. These implementations are generally more efficient,
/// at the cost of readability.
pub struct Element;
