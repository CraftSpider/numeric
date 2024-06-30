//! Various scalar 'real' value type implementation

#![cfg_attr(not(feature = "std"), no_std)]

pub mod decimal;
pub mod fixed;
pub mod f;
pub mod p;

pub use decimal::Decimal;
pub use fixed::Fixed;
pub use f::F;
