//! Various integral type implementations. Signed, unsigned, and unbounded.
//!
//! ### Note
//! Unbounded, or 'big' integers, are only available on crate feature `alloc`.

#![no_std]
#[cfg(feature = "std")]
extern crate alloc;

#[cfg(feature = "std")]
mod big_int;
mod i;
mod u;
// #[cfg(test)]
// mod tests;

#[cfg(feature = "std")]
pub use big_int::BigInt;
pub use i::I;
pub use u::U;
