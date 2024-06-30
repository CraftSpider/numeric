//! Various integral type implementations. Signed, unsigned, and unbounded.
//!
//! ### Note
//! Unbounded, or 'big' integers, are only available on crate feature `alloc`.

#![no_std]
#[cfg(feature = "std")]
extern crate alloc;

mod i;
mod u;
#[cfg(feature = "std")]
mod big_int;
// #[cfg(test)]
// mod tests;

#[cfg(feature = "std")]
pub use big_int::BigInt;
pub use u::U;
pub use i::I;
