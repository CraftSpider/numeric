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

use crate::bit_slice::{BitLike, BitOwned, BitSlice};
use numeric_traits::class::Bounded;
use numeric_traits::identity::Zero;

/// Collections of BitOwned values, like tuples or arrays
pub trait MultiBitOwned {
    /// Bit type of owned values
    type Bit: BitLike;

    /// Fill all values with the requested bit value
    fn fill(&mut self, val: Self::Bit);
}

impl<O: BitOwned> MultiBitOwned for O {
    type Bit = O::Bit;

    fn fill(&mut self, val: Self::Bit) {
        self.iter_mut().for_each(|v| *v = val);
    }
}

impl<O: BitOwned> MultiBitOwned for (O, O) {
    type Bit = O::Bit;

    fn fill(&mut self, val: Self::Bit) {
        self.0.iter_mut().for_each(|v| *v = val);
        self.1.iter_mut().for_each(|v| *v = val);
    }
}

/// Algorithm types. Add, Sub, Div, DivRem, etc.
pub trait AlgoTy {
    /// Whether saturation fills the value with MAX or 0.
    const SATURATE_HIGH: bool;

    /// Outputs for this algorithm. As an example, `O` or `(O, O)`.
    type Out<O: BitOwned>: MultiBitOwned<Bit = O::Bit>;
}

/// Algorithms implementing an 'add' operation
pub struct Add;

/// Algorithm implementing a 'sub' operation
pub struct Sub;

/// Algorithm implementing a 'mul' operation
pub struct Mul;

/// Algorithm implementing a 'div' operation
pub struct Div;

/// Algorithm implemented a 'rem' operation
pub struct Rem;

/// Algorithm implementing a 'div-rem' operation
pub struct DivRem;

impl AlgoTy for Add {
    const SATURATE_HIGH: bool = true;
    type Out<O: BitOwned> = O;
}

impl AlgoTy for Sub {
    const SATURATE_HIGH: bool = false;
    type Out<O: BitOwned> = O;
}

impl AlgoTy for Mul {
    const SATURATE_HIGH: bool = true;
    type Out<O: BitOwned> = O;
}

impl AlgoTy for Div {
    const SATURATE_HIGH: bool = false;
    type Out<O: BitOwned> = O;
}

impl AlgoTy for Rem {
    const SATURATE_HIGH: bool = false;
    type Out<O: BitOwned> = O;
}

impl AlgoTy for DivRem {
    const SATURATE_HIGH: bool = false;
    type Out<O: BitOwned> = (O, O);
}

/// Trait for algorithm implementations.
///
/// The generic on the trait is the actual type of math algorithm to implement - for example,
/// `Add` or `DivRem`. This type can control the number of outputs - for example, div-rem can output
/// both values at once for efficiency of calculation.
///
/// Each method generally take a left and right bit-slice type, and an owned output type. This type
/// is used for both the final output, and any extra intermediate buffers needed to calculate the
/// result. This means it may be created more than once, though in general a minimal number of
/// intermediate buffers will be used.
///
/// # Examples
///
/// ```
/// # use numeric_bits::algos::{Algo, Add, Element};
///
/// // The output type will often need annotated
/// let out: [_; _] = <Element as Algo<Add>>::wrapping(&[1u32], &[1]);
/// assert_eq!(out, [2]);
/// ```
pub trait Algo<A: AlgoTy> {
    /// Calculate the result using overflowing logic. That means wrapping, with an extra boolean to
    /// indicate wraparound.
    fn overflowing<O, L, R>(left: &L, right: &R) -> (A::Out<O>, bool)
    where
        O: BitOwned<Bit = L::Bit>,
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>;

    /// Calculate the result using checked logic. That means a value is only returned if no wrapping
    /// would occur.
    fn checked<O, L, R>(left: &L, right: &R) -> Option<A::Out<O>>
    where
        O: BitOwned<Bit = L::Bit>,
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        let (val, overflow) = Self::overflowing(left, right);
        if overflow {
            None
        } else {
            Some(val)
        }
    }

    /// Calculate the value using wrapping logic. On overflow or underflow, the value jumps to the
    /// opposite end.
    fn wrapping<O, L, R>(left: &L, right: &R) -> A::Out<O>
    where
        O: BitOwned<Bit = L::Bit>,
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        Self::overflowing(left, right).0
    }

    /// Calculate the value using saturating logic. On overflow or underflow, the value is set to
    /// the maximum or minimum value respectively.
    fn saturating<O, L, R>(left: &L, right: &R) -> A::Out<O>
    where
        O: BitOwned<Bit = L::Bit>,
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        let (mut val, overflow) = Self::overflowing(left, right);
        if overflow {
            if A::SATURATE_HIGH {
                val.fill(L::Bit::max_value());
            } else {
                val.fill(L::Bit::zero());
            }
            val
        } else {
            val
        }
    }

    /// Calculate the result using overflowing logic, putting the result into an exising output.
    fn overflowing_into<L, R, O>(left: &L, right: &R, out: &mut A::Out<O>) -> bool
    where
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
        O: BitOwned<Bit = L::Bit>;

    /// Calculate the result using overflowing logic, putting the result into an existing output
    fn wrapping_into<L, R, O>(left: &L, right: &R, out: &mut A::Out<O>)
    where
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
        O: BitOwned<Bit = L::Bit>,
    {
        Self::overflowing_into(left, right, out);
    }
}

/// Trait for assignment algorithm implementations. These algorithms can be implemented to
/// efficiently re-use one of their input buffers as an output.
///
/// The generic on the trait is the actual type of math algorithm to implement - for example,
/// `Add` or `DivRem`. This type can control the number of outputs - for example, div-rem can output
/// both values at once for efficiency of calculation.
///
/// Each method generally take a left and right bit-slice type, and an owned buffer type. This type
/// is used for any extra intermediate buffers needed to calculate the result. This means it may be
/// created more than once, though in general a minimal number of intermediate buffers will be used.
pub trait AssignAlgo<A: AlgoTy> {
    /// Calculate the result using overflowing logic. That means wrapping, with an extra boolean to
    /// indicate wraparound. The result will be written into the left-side slice.
    fn overflowing<O, L, R>(left: &mut L, right: &R) -> bool
    where
        O: BitOwned<Bit = L::Bit>,
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>;

    /// Calculate the result using checked logic. That means a value is only returned if no wrapping
    /// would occur.
    fn checked<O, L, R>(left: &mut L, right: &R) -> Option<()>
    where
        O: BitOwned<Bit = L::Bit>,
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        let overflow = Self::overflowing::<O, L, R>(left, right);
        if overflow {
            None
        } else {
            Some(())
        }
    }

    /// Calculate the value using wrapping logic. On overflow or underflow, the value jumps to the
    /// opposite end.
    fn wrapping<O, L, R>(left: &mut L, right: &R)
    where
        O: BitOwned<Bit = L::Bit>,
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        Self::overflowing::<O, L, R>(left, right);
    }

    /// Calculate the value using saturating logic. On overflow or underflow, the value is set to
    /// the maximum or minimum value respectively.
    fn saturating<O, L, R>(left: &mut L, right: &R)
    where
        O: BitOwned<Bit = L::Bit>,
        L: ?Sized + BitSlice,
        R: ?Sized + BitSlice<Bit = L::Bit>,
    {
        let overflow = Self::overflowing::<O, L, R>(left, right);
        if overflow {
            if A::SATURATE_HIGH {
                left.iter_mut().for_each(|v| *v = O::Bit::max_value());
            } else {
                left.iter_mut().for_each(|v| *v = O::Bit::zero());
            }
        }
    }
}

pub use bits::*;
pub use cmp::*;
pub use div_rem::*;
pub use shift::*;

/// Simple bitwise implementations of algorithms. These implementations are generally inefficient,
/// but straightforward compared to alternative approaches.
pub struct Bitwise;

/// Element-wise implementations of algorithms. These implementations are generally more efficient,
/// at the cost of readability.
pub struct Element;
