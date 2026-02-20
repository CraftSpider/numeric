//! Low-level bit and integer slice manipulation facilities. The interface exposed by this crate
//! will change much more frequently than higher-level crates such as `numeric_ints`.
//!
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod algos;
pub mod array;
pub mod bit_slice;
pub mod endian;
