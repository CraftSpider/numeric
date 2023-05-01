//! Central trait for the `numeric` family of crates
//!
//! # What is `numeric`?
//!
//! `numeric` is a collection of number-related types and implementations. This include an efficient
//! implementation of a big integer, vector and matrix math, implementations for fixed-point
//! and decimal fraction real numbers, and more.
//!

mod int {
    pub use numeric_ints::*;
}

mod real {
    pub use numeric_reals::*;
}

mod compound {
    pub use numeric_compounds::*;
}
