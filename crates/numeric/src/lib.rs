//! Central crate for the `numeric` family of crates
//!
//! # What is `numeric`?
//!
//! `numeric` is a collection of number-related types and implementations. This includes an efficient
//! implementation of a big integer, vector and matrix math, implementations for fixed-point
//! and rational real numbers, and more.
//!

#[doc(inline)]
pub use numeric_ints as int;

#[doc(inline)]
pub use numeric_reals as real;

#[doc(inline)]
pub use numeric_compounds as compound;

#[doc(inline)]
pub use numeric_traits as traits;
