//! Traits for converting types to/from byte representations

/// Trait for types that can be made from or converted to raw bytes, in different endians.
pub trait ConvertBytes<const N: usize>: Sized {
    /// Create a value from native-endian bytes
    fn from_ne_bytes(bytes: [u8; N]) -> Self {
        if cfg!(target_endian = "little") {
            Self::from_le_bytes(bytes)
        } else {
            Self::from_be_bytes(bytes)
        }
    }

    /// Create a value from little-endian bytes
    fn from_le_bytes(bytes: [u8; N]) -> Self;

    /// Create a value from big-endian bytes
    fn from_be_bytes(bytes: [u8; N]) -> Self;

    /// Convert this value into native-endian bytes
    fn to_ne_bytes(self) -> [u8; N] {
        if cfg!(target_endian = "little") {
            Self::to_le_bytes(self)
        } else {
            Self::to_be_bytes(self)
        }
    }

    /// Convert this value into little-endian bytes
    fn to_le_bytes(self) -> [u8; N];

    /// Convert this value into big-endian bytes
    fn to_be_bytes(self) -> [u8; N];
}
