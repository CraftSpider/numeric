//! Various big integer algorithms.
//!
//! Algorithms that work on [bit-slice](crate::bit_slice) types to perform various mathematical
//! operations on arbitrary length, or even unbounded, integers. This provides a low-level base on
//! which to build various integer types not natively available in Rust.

mod add;
mod bits;
mod cmp;
mod div_rem;
mod mul;
mod shift;
mod sub;

pub use add::*;
pub use bits::*;
pub use cmp::*;
pub use div_rem::*;
pub use mul::*;
pub use shift::*;
pub use sub::*;

pub struct Bitwise;

pub struct Element;
