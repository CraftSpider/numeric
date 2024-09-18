//! Central trait for the `numeric` family of crates
//!
//! # What is `numeric`?
//!
//! `numeric` is a collection of number-related types and implementations. This includes an efficient
//! implementation of a big integer, vector and matrix math, implementations for fixed-point
//! and rational real numbers, and more.
//!

pub mod int {
    pub use numeric_ints::*;
}

pub mod real {
    pub use numeric_reals::*;
}

pub mod compound {
    pub use numeric_compounds::*;
}

pub mod traits {
    pub use numeric_traits::*;
}
