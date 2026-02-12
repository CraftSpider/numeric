//! Traits for representing various capabilities of numeric types
//!
//! Part of the `numeric` project

#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub mod bytes;
pub mod cast;
pub mod class;
pub mod identity;
pub mod ops;

mod __impl;
