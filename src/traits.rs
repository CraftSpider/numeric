use num_traits::PrimInt;
use crate::big_int::BigInt;

/// Trait to be implemented by any integral type. If implemented incorrectly, some types may behave
/// strangely (See [`Decimal`][crate::decimal::Decimal]
pub trait Integral {}

impl<T: PrimInt> Integral for T {}
impl Integral for BigInt {}
