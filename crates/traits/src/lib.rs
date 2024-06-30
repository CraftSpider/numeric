//! Traits for representing various capabilities of numeric types
//!
//! Part of the `numeric` project

#![cfg_attr(not(feature = "std"), no_std)]

pub mod bytes;
pub mod cast;
pub mod class;
pub mod identity;
pub mod ops;

mod __impl;
