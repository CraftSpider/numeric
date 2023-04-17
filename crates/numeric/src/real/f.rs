//! N-byte bounded floating point value

use crate::int::u::U;

/// N-byte floating point value. `F<4> == f32`, `F<8> == f64`, etc.
///
/// Represented as 1 sign bit, (3N - 1) exponent bits, and all remaining mantissa bits.
pub struct F<const N: usize>();

impl<const N: usize> F<N> {
    /// Convert this float into raw parts - boolean sign bit, and `U<N>` sized exponent and mantissa
    #[must_use]
    pub const fn into_raw_parts(self) -> (bool, U<N>, U<N>) {
        todo!()
    }
}
