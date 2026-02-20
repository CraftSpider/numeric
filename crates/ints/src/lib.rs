//! Various integral type implementations. Signed, unsigned, and unbounded.
//!
//! ### Note
//! Unbounded, or 'big' integers, are only available on crate feature `alloc`.

#![no_std]
#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
#[macro_use]
mod big_utils;
mod i;
#[cfg(feature = "alloc")]
mod ibig;
mod u;
#[cfg(feature = "alloc")]
mod ubig;
// #[cfg(test)]
// mod tests;

pub use i::I;
#[cfg(feature = "alloc")]
pub use ibig::IBig;
pub use u::U;
#[cfg(feature = "alloc")]
pub use ubig::UBig;
