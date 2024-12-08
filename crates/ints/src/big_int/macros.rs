macro_rules! impl_assign_for_int {
    ($ty:ty) => {
        impl_assign_for_int!($ty, +, AddAssign, add_assign);
        impl_assign_for_int!($ty, -, SubAssign, sub_assign);
        impl_assign_for_int!($ty, *, MulAssign, mul_assign);
        impl_assign_for_int!($ty, /, DivAssign, div_assign);
        impl_assign_for_int!($ty, %, RemAssign, rem_assign);
    };
    ($ty:ty, $op:tt, $trait:ident, $meth:ident) => {
        impl core::ops::$trait<$ty> for BigInt {
            fn $meth(&mut self, other: $ty) {
                *self = &*self $op BigInt::from(other);
            }
        }
    };
}

macro_rules! impl_ops_for_int {
    ($ty:ty) => {
        impl_ops_for_int!($ty, +, Add, add);
        impl_ops_for_int!($ty, -, Sub, sub);
        impl_ops_for_int!($ty, *, Mul, mul);
        impl_ops_for_int!($ty, /, Div, div);
        impl_ops_for_int!($ty, %, Rem, rem);

        impl_ops_for_int!($ty, <<, Shl, shl);
        impl_ops_for_int!($ty, >>, Shr, shr);
    };

    ($ty:ty, $op:tt, $trait:ident, $meth:ident) => {
        impl core::ops::$trait<$ty> for BigInt {
            type Output = BigInt;

            fn $meth(self, other: $ty) -> BigInt {
                self $op BigInt::from(other)
            }
        }
    };
}

macro_rules! impl_for_int {
    ($signed:ty, $unsigned:ty) => {
        // From/TryFrom

        impl From<$signed> for BigInt {
            fn from(val: $signed) -> Self {
                let neg = val.is_negative();
                BigInt::new_slice::<&[usize]>(
                    &int_to_arr::<$unsigned, usize, { arr_size::<$unsigned>() }>(
                        val.unsigned_abs(),
                    ),
                    neg,
                )
            }
        }

        impl From<$unsigned> for BigInt {
            fn from(val: $unsigned) -> Self {
                BigInt::new_slice::<&[usize]>(
                    &int_to_arr::<$unsigned, usize, { arr_size::<$unsigned>() }>(val),
                    false,
                )
            }
        }

        impl TryFrom<BigInt> for $signed {
            type Error = OutOfRangeError;

            fn try_from(bi: BigInt) -> Result<Self, Self::Error> {
                <$signed as TryFrom<_>>::try_from(&bi)
            }
        }

        impl TryFrom<BigInt> for $unsigned {
            type Error = OutOfRangeError;

            fn try_from(bi: BigInt) -> Result<Self, Self::Error> {
                <$unsigned as TryFrom<_>>::try_from(&bi)
            }
        }

        impl TryFrom<&BigInt> for $signed {
            type Error = OutOfRangeError;

            fn try_from(bi: &BigInt) -> Result<Self, Self::Error> {
                if bi > &BigInt::from(Self::MAX) {
                    Err(OutOfRangeError::above())
                } else if bi < &BigInt::from(Self::MIN) {
                    Err(OutOfRangeError::below())
                } else {
                    bi.with_slice(|s| arr_to_int(s))
                        .ok_or_else(|| OutOfRangeError::above())
                }
            }
        }

        impl TryFrom<&BigInt> for $unsigned {
            type Error = OutOfRangeError;

            fn try_from(bi: &BigInt) -> Result<Self, Self::Error> {
                if bi > &BigInt::from(Self::MAX) {
                    Err(OutOfRangeError::above())
                } else if bi < &BigInt::from(Self::MIN) {
                    Err(OutOfRangeError::below())
                } else {
                    bi.with_slice(|s| arr_to_int(s))
                        .ok_or_else(|| OutOfRangeError::above())
                }
            }
        }

        // Casts

        impl numeric_traits::cast::FromTruncating<BigInt> for $unsigned {
            fn truncate_from(val: BigInt) -> Self {
                val.with_slice(|s| arr_to_int(s))
                    .unwrap_or(<$unsigned>::MAX)
            }
        }

        impl numeric_traits::cast::FromTruncating<BigInt> for $signed {
            fn truncate_from(val: BigInt) -> Self {
                val.with_slice(|s| arr_to_int(s)).unwrap_or(<$signed>::MAX)
                    * if val.is_negative() { -1 } else { 1 }
            }
        }

        impl numeric_traits::cast::FromChecked<BigInt> for $unsigned {
            fn from_checked(val: BigInt) -> Option<Self> {
                val.try_into().ok()
            }
        }

        impl numeric_traits::cast::FromChecked<BigInt> for $signed {
            fn from_checked(val: BigInt) -> Option<Self> {
                val.try_into().ok()
            }
        }

        impl numeric_traits::cast::FromSaturating<BigInt> for $unsigned {
            fn saturate_from(val: BigInt) -> Self {
                match val.try_into() {
                    Ok(val) => val,
                    Err(OutOfRangeError(Side::Above)) => Self::MAX,
                    Err(OutOfRangeError(Side::Below)) => Self::MIN,
                }
            }
        }

        impl numeric_traits::cast::FromSaturating<BigInt> for $signed {
            fn saturate_from(val: BigInt) -> Self {
                match val.try_into() {
                    Ok(val) => val,
                    Err(OutOfRangeError(Side::Above)) => Self::MAX,
                    Err(OutOfRangeError(Side::Below)) => Self::MIN,
                }
            }
        }

        // Comparison

        impl PartialEq<$signed> for BigInt {
            fn eq(&self, other: &$signed) -> bool {
                if self.is_negative() != other.is_negative() {
                    return false;
                }
                let other = other.abs();

                self.with_slice(|this| {
                    let arr = int_to_arr::<_, _, { arr_size::<$unsigned>() }>(other as $unsigned);
                    this == IntSlice::shrink(&arr as &[_])
                })
            }
        }

        impl PartialEq<$unsigned> for BigInt {
            fn eq(&self, other: &$unsigned) -> bool {
                self.with_slice(|this| {
                    let arr = int_to_arr::<_, _, { arr_size::<$unsigned>() }>(*other);
                    this == IntSlice::shrink(&arr as &[_])
                })
            }
        }

        impl PartialOrd<$signed> for BigInt {
            fn partial_cmp(&self, other: &$signed) -> Option<Ordering> {
                Some(BigInt::cmp(self, &BigInt::from(*other)))
            }
        }

        impl PartialOrd<$unsigned> for BigInt {
            fn partial_cmp(&self, other: &$unsigned) -> Option<Ordering> {
                Some(BigInt::cmp(self, &BigInt::from(*other)))
            }
        }

        // Operations

        impl_ops_for_int!($signed);
        impl_ops_for_int!($unsigned);
        impl_assign_for_int!($signed);
        impl_assign_for_int!($unsigned);
    };
}

macro_rules! impl_op {
    (add($self:ident, $rhs:ident) => $block:block) => {
        impl_op!(add, Add, $self, $rhs, $block);
    };
    (sub($self:ident, $rhs:ident) => $block:block) => {
        impl_op!(sub, Sub, $self, $rhs, $block);
    };
    (mul($self:ident, $rhs:ident) => $block:block) => {
        impl_op!(mul, Mul, $self, $rhs, $block);
    };
    (div($self:ident, $rhs:ident) => $block:block) => {
        impl_op!(div, Div, $self, $rhs, $block);
    };
    (rem($self:ident, $rhs:ident) => $block:block) => {
        impl_op!(rem, Rem, $self, $rhs, $block);
    };
    (shl($self:ident, $rhs:ident) => $block:block) => {
        impl_op!(shl, Shl, $self, $rhs, $block);
    };
    (shr($self:ident, $rhs:ident) => $block:block) => {
        impl_op!(shr, Shr, $self, $rhs, $block);
    };
    (bitand($self:ident, $rhs:ident) => $block:block) => {
        impl_op!(bitand, BitAnd, $self, $rhs, $block);
    };
    (bitor($self:ident, $rhs:ident) => $block:block) => {
        impl_op!(bitor, BitOr, $self, $rhs, $block);
    };
    (bitxor($self:ident, $rhs:ident) => $block:block) => {
        impl_op!(bitxor, BitXor, $self, $rhs, $block);
    };
    ($meth:ident, $trait:ident, $self:ident, $rhs:ident, $block:block) => {
        impl core::ops::$trait<BigInt> for BigInt {
            type Output = BigInt;

            fn $meth(self, rhs: BigInt) -> Self::Output {
                <&BigInt as core::ops::$trait<&BigInt>>::$meth(&self, &rhs)
            }
        }

        impl core::ops::$trait<&BigInt> for BigInt {
            type Output = BigInt;

            fn $meth(self, rhs: &BigInt) -> Self::Output {
                <&BigInt as core::ops::$trait<&BigInt>>::$meth(&self, rhs)
            }
        }

        impl core::ops::$trait<BigInt> for &BigInt {
            type Output = BigInt;

            fn $meth(self, rhs: BigInt) -> Self::Output {
                <&BigInt as core::ops::$trait<&BigInt>>::$meth(self, &rhs)
            }
        }

        impl core::ops::$trait<&BigInt> for &BigInt {
            type Output = BigInt;

            fn $meth($self, $rhs: &BigInt) -> Self::Output $block
        }
    };
}
