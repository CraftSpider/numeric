//! N-byte bounded floating point value

#[cfg(feature = "ints")]
use numeric_ints::U;

/// N-byte floating point value. `F<4> == f32`, `F<8> == f64`, etc.
///
/// Represented as 1 sign bit, `3N - 1` exponent bits, and all remaining (`5N`) mantissa bits.
pub struct F<const N: usize>([u8; N]);

impl<const N: usize> F<N> {
    // Mantissa Bytes: ⌈5N / 8⌉
    // Exponent Bytes: ⌈(3N - 1) / 8⌉
    // TODO: This should have correctly sized mantissa and exponent arrays
    // /// Convert this float into raw parts - boolean sign bit, and `[u8; N]` sized exponent and mantissa,
    // /// both having trailing zeros for the unused portions.
    // #[must_use]
    // pub const fn into_raw_parts(self) -> (bool, [u8; N], [u8; N]) {
    //     f32::raw_parts();
    //     todo!()
    // }

    #[cfg(feature = "ints")]
    pub fn to_bits(self) -> U<N> {
        U::from_le_bytes(self.to_le_bytes())
    }

    #[cfg(feature = "ints")]
    pub fn from_bits(val: U<N>) -> Self {
        F(val.to_le_bytes())
    }

    /// Create a value from raw bytes, laid out in little-endian order
    #[must_use]
    pub const fn from_le_bytes(bytes: [u8; N]) -> F<N> {
        F(bytes)
    }

    /// Create a value from raw bytes, laid out in big-endian order
    #[must_use]
    pub fn from_be_bytes(mut bytes: [u8; N]) -> F<N> {
        bytes.reverse();
        F(bytes)
    }

    /// Create a value from raw bytes, laid out in the native endianness
    #[must_use]
    pub fn from_ne_bytes(bytes: [u8; N]) -> F<N> {
        if cfg!(target_endian = "little") {
            Self::from_le_bytes(bytes)
        } else {
            Self::from_be_bytes(bytes)
        }
    }

    /// Convert this value to raw bytes, laid out in little-endian order
    #[must_use]
    pub const fn to_le_bytes(self) -> [u8; N] {
        self.0
    }

    /// Convert this value to raw bytes, laid out in big-endian order
    #[must_use]
    pub fn to_be_bytes(mut self) -> [u8; N] {
        self.0.reverse();
        self.0
    }

    /// Convert this value to raw bytes, laid out in the native endianness
    #[must_use]
    pub fn to_ne_bytes(self) -> [u8; N] {
        if cfg!(target_endian = "little") {
            self.to_le_bytes()
        } else {
            self.to_be_bytes()
        }
    }
}
