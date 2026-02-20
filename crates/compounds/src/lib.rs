//! Various mathematical compound type implementations - complex numbers, matrices, etc

#![no_std]
#![feature(min_generic_const_args)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
pub mod bivec;
pub mod complex;
pub mod matrix;
#[cfg(feature = "alloc")]
pub mod rotor;
pub mod vector;
