//! Various mathematical compound type implementations - complex numbers, matrices, etc

#![no_std]

#[cfg(feature = "std")]
extern crate alloc;

#[cfg(feature = "std")]
pub mod bivec;
pub mod complex;
pub mod matrix;
pub mod rotor;
pub mod vector;
