//! Traits for representing various capabilities of numeric types
//!
//! Part of the `numeric` project

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
)]

pub mod bytes;
pub mod cast;
pub mod class;
pub mod identity;
pub mod ops;

mod __impl;
