use crate::bivec::BiVector;
use crate::vector::Vector;

#[allow(dead_code)]
pub struct Rotor<T, const DIM: usize> {
    product: Vector<T, DIM>,
    bivec: BiVector<T, DIM>,
}
