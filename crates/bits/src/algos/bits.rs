use crate::bit_slice::BitSliceExt;

mod impls;

pub trait BitAlgo {
    fn and<'a, L, R>(left: &L, right: &R, out: &'a mut [L::Bit]) -> &'a [L::Bit]
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>;

    fn or<'a, L, R>(left: &L, right: &R, out: &'a mut [L::Bit]) -> &'a [L::Bit]
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>;

    fn xor<'a, L, R>(left: &L, right: &R, out: &'a mut [L::Bit]) -> &'a [L::Bit]
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>;

    fn not<'a, L>(left: &L, out: &'a mut [L::Bit]) -> &'a [L::Bit]
    where
        L: ?Sized + BitSliceExt;
}

pub trait AssignBitAlgo {
    fn and<L, R>(left: &mut L, right: &R)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>;

    fn or<L, R>(left: &mut L, right: &R)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>;

    fn xor<L, R>(left: &mut L, right: &R)
    where
        L: ?Sized + BitSliceExt,
        R: ?Sized + BitSliceExt<Bit = L::Bit>;

    fn not<L>(left: &mut L)
    where
        L: ?Sized + BitSliceExt;
}
