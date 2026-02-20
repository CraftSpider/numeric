//! Various big integer algorithms.
//!
//! Algorithms that work on [bit-slice](crate::bit_slice) types to perform various mathematical
//! operations on arbitrary length, or even unbounded, integers. This provides a low-level base on
//! which to build various integer types not natively available in Rust.

#[allow(missing_docs)]
mod add;
#[allow(missing_docs)]
mod bits;
#[allow(missing_docs)]
mod cmp;
#[allow(missing_docs)]
mod div_rem;
#[allow(missing_docs)]
mod mul;
#[allow(missing_docs)]
mod shift;
#[allow(missing_docs)]
mod sub;

pub use add::*;
pub use bits::*;
pub use cmp::*;
pub use div_rem::*;
pub use mul::*;
pub use shift::*;
pub use sub::*;

/// Simple bitwise implementations of algorithms. These implementations are generally inefficient,
/// but straightforward compared to alternative approaches.
pub struct Bitwise;

/// Element-wise implementations of algorithms. These implementations are generally more efficient,
/// at the cost of readability.
pub struct Element;
