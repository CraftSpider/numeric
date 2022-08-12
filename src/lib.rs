//! Crate for implementations of numeric types - `BigInt`, `Decimal`, etc.

#![feature(array_zip)]

#![cfg_attr(feature = "specialize", feature(min_specialization))]
#![cfg_attr(feature = "__bench_priv", allow(unused))]

#![warn(
    missing_docs,
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
pub mod bit_slice;
pub mod big_int;
pub mod u;
pub mod i;
