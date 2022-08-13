use crate::u::U;

/// N-byte floating point value. `F<4> == f32`, `F<8> == f64`, etc.
///
/// Represented as 1 sign bit, (3N - 1) exponent bits, and all remaining mantissa bits.
pub struct F<const N: usize>();

impl<const N: usize> F<N> {
    pub const fn into_raw_parts(self) -> (bool, U<N>, U<N>) {
        todo!()
    }
}
