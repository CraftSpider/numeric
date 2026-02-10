mod bitwise;
mod element;

use crate::bit_slice::BitSliceExt;
pub use bitwise::*;
pub use element::*;
// `Default: BinAlg<Add, Growing>`
// `Bitwise: BinAlg<Div, Wrapping>`

pub trait Operation {
    type Out<L>;
    type Extra<L>;
}

pub struct Add;

impl Operation for Add {
    type Out<L> = L;
    type Extra<L> = ();
}

pub struct Sub;

impl Operation for Sub {
    type Out<L> = L;
    type Extra<L> = ();
}

pub struct Mul;

impl Operation for Mul {
    type Out<L> = L;
    type Extra<L> = ();
}

pub struct DivRem;

impl Operation for DivRem {
    type Out<L> = (L, L);
    type Extra<L> = L;
}

pub trait BinAlg<OP: Operation> {
    #[cfg(feature = "std")]
    fn growing<L, R>(left: &L, right: &R) -> OP::Out<alloc::vec::Vec<L::Bit>>
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>;

    fn overflowing<'a, L, R>(
        left: &L,
        right: &R,
        out: OP::Out<&'a mut [L::Bit]>,
    ) -> (OP::Out<&'a [L::Bit]>, bool)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>;

    fn wrapping<'a, L, R>(
        left: &L,
        right: &R,
        out: OP::Out<&'a mut [L::Bit]>,
    ) -> OP::Out<&'a [L::Bit]>
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        Self::overflowing(left, right, out).0
    }

    fn checked<'a, L, R>(
        left: &L,
        right: &R,
        out: OP::Out<&'a mut [L::Bit]>,
    ) -> Option<OP::Out<&'a [L::Bit]>>
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        let (out, overflow) = Self::overflowing(left, right, out);
        if overflow {
            None
        } else {
            Some(out)
        }
    }

    fn saturating<'a, L, R>(
        left: &L,
        right: &R,
        out: OP::Out<&'a mut [L::Bit]>,
    ) -> OP::Out<&'a [L::Bit]>
    where
        L: BitSliceExt,
        R: BitSliceExt<Bit = L::Bit>;
}

pub trait AssignBinAlg<OP: Operation> {
    fn overflowing<L, R>(left: &mut L, right: &R, extra: OP::Extra<&mut [L::Bit]>) -> bool
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>;

    fn wrapping<L, R>(left: &mut L, right: &R, extra: OP::Extra<&mut [L::Bit]>)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>,
    {
        Self::overflowing(left, right, extra);
    }

    fn saturating<L, R>(left: &L, right: &R, out: OP::Extra<&mut [L::Bit]>)
    where
        L: BitSliceExt,
        R: BitSliceExt<Bit = L::Bit>;
}

// The choice or &mut L is kind of odd - it makes usage complicated. You can't re-use a value
// you add to another. Should probably swap to `(&L, &R, &mut O) -> &O`. Then items can pass in
// the desired output memory.

// Long division is odd because it's actually doing two things at once - both remainder and divisor.
// It really just wants to be given two scratch pads, one for each output it generates.

// a +,-,* b
//   growing: (&L, &R) -> Vec<B>
//   overflowing: (&mut L, &R) -> (&L, bool)
//   wrapping: (&mut L, &R) -> &L
//   checked: (&mut L, &R) -> Option<&L> - left in indeterminate state
//   saturating: (&mut L, &R) -> &L

// Odd ones out:

// a /,% b
//   growing: (&L, &R) -> Vec<B>
//   overflowing: (&mut L, &R, rem: &mut [B]) -> (&L, bool)
//   wrapping: (&mut L, &R, rem: &mut [B]) -> &L
//   checked: (&mut L, &R, rem: &mut [B]) -> Option<&L>
//   saturating: (&mut L, &R, rem: &mut [B]) -> &L

// Can we make this just use `&R`? Probably, since we can either
// truncate it or just saturate to 0.
// a <<,>> b
//   growing: (&L, usize) -> Vec<B>
//   overflowing: (&mut L, usize) -> (&L, bool)
//   wrapping: (&mut L, usize) -> &L
//   checked: (&mut L, usize) -> Option<&L>
//   saturating: (&mut L, usize) -> &L
