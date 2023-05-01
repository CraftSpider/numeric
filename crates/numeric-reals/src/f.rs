//! N-byte bounded floating point value

/// N-byte floating point value. `F<4> == f32`, `F<8> == f64`, etc.
///
/// Represented as 1 sign bit, `3N - 1` exponent bits, and all remaining (`5N`) mantissa bits.
pub struct F<const N: usize>();

impl<const N: usize> F<N> {
    // Mantissa Bytes: ⌈5N / 8⌉
    // Exponent Bytes: ⌈(3N - 1) / 8⌉
    // TODO: This should have correctly sized mantissa and exponent arrays
    /// Convert this float into raw parts - boolean sign bit, and `[u8; N]` sized exponent and mantissa,
    /// both having trailing zeros for the unused portions.
    #[must_use]
    pub const fn into_raw_parts(self) -> (bool, [u8; N], [u8; N]) {
        todo!()
    }
}
