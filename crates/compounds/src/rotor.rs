//! Rotors, N-dimensional generalization of vector rotations. Isomorphic to quaternions in 3
//! dimensions.

use crate::bivec::BiVector;
use crate::vector::Vector;

/// N-dimensional rotation primitive.
///
/// Represents the rotation of a vector around a number of planes dependent on the dimension, but
/// always sufficient to represent any possible rotation.
#[allow(dead_code)]
pub struct Rotor<T, const DIM: usize> {
    product: Vector<T, DIM>,
    bivec: BiVector<T, DIM>,
}
