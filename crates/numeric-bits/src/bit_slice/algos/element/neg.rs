use crate::bit_slice::BitSliceExt;

pub trait ElementNot: BitSliceExt {
    fn not(this: &mut Self) {
        this.slice_mut()
            .iter_mut()
            .for_each(|v| *v = !*v)
    }
}

impl<T> ElementNot for T
where
    T: ?Sized + BitSliceExt,
{}
