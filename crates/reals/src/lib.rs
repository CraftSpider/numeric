//! Various scalar 'real' value type implementation

#![cfg_attr(not(feature = "std"), no_std)]

pub mod decimal;
pub mod f;
pub mod fixed;
pub mod p;

pub use decimal::Decimal;
pub use f::F;
pub use fixed::Fixed;
