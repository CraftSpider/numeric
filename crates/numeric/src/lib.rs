//! Crate for implementations of numeric types - `BigInt`, `Decimal`, etc.

#![feature(array_zip)]
#![feature(array_methods)]

#![cfg_attr(feature = "specialize", feature(min_specialization))]
#![cfg_attr(feature = "__bench_priv", allow(unused))]

#![warn(
    // missing_docs,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    missing_abi,
    noop_method_call,
    pointer_structural_match,
    semicolon_in_expressions_from_macros,
    unused_import_braces,
    unused_lifetimes,
    clippy::missing_panics_doc,
    clippy::doc_markdown,
    clippy::ptr_as_ptr,
    clippy::cloned_instead_of_copied,
    clippy::unreadable_literal,
    clippy::must_use_candidate
)]

#[macro_use]
mod macros;
mod intern;
bench_public! { mod utils; }

/// Collection of various integer type implementations
pub mod int {
    pub mod big_int;
    pub mod u;
    pub mod i;

    pub use big_int::BigInt;
    pub use u::U;
    pub use i::I;
}

/// Collection of various scalar 'real' value type implementation
pub mod real {
    pub mod decimal;
    pub mod fixed;
    pub mod f;

    pub use decimal::Decimal;
    pub use fixed::Fixed;
    pub use f::F;
}

/// Collection of various mathematical compound type implementations - complex numbers, matrices,
/// etc
pub mod compound {
    pub mod complex;
    pub mod matrix;
    pub mod vector;
    pub mod bivec;
    pub mod rotor;
}

pub mod traits;
pub mod bit_slice;
