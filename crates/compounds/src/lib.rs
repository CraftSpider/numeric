//! Various mathematical compound type implementations - complex numbers, matrices, etc

#![no_std]

#[cfg(feature = "std")]
extern crate alloc;

pub mod complex;
pub mod matrix;
pub mod vector;
#[cfg(feature = "std")]
pub mod bivec;
pub mod rotor;
