macro_rules! impl_assign_for_int {
    ($ty:ty) => {
        impl_assign_for_int!($ty, +, AddAssign, add_assign);
        impl_assign_for_int!($ty, -, SubAssign, sub_assign);
        impl_assign_for_int!($ty, *, MulAssign, mul_assign);
        impl_assign_for_int!($ty, /, DivAssign, div_assign);
        impl_assign_for_int!($ty, %, RemAssign, rem_assign);
    };
    ($ty:ty, $op:tt, $trait:ident, $meth:ident) => {
        impl core::ops::$trait<$ty> for IBig {
            fn $meth(&mut self, other: $ty) {
                *self = &*self $op IBig::from(other);
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
        impl core::ops::$trait<$ty> for IBig {
            type Output = IBig;

            fn $meth(self, other: $ty) -> IBig {
                self $op IBig::from(other)
            }
        }
    };
}

macro_rules! impl_for_int {
    ($signed:ty, $unsigned:ty) => {
        // From/TryFrom

        impl From<$signed> for IBig {
            fn from(val: $signed) -> Self {
                let neg = val.is_negative();
                IBig::new_slice::<&[usize]>(
                    &int_to_arr::<$unsigned, usize, { arr_size::<$unsigned>() }>(
                        val.unsigned_abs(),
                    ),
                    neg,
                )
            }
        }

        impl From<$unsigned> for IBig {
            fn from(val: $unsigned) -> Self {
                IBig::new_slice::<&[usize]>(
                    &int_to_arr::<$unsigned, usize, { arr_size::<$unsigned>() }>(val),
                    false,
                )
            }
        }

        impl TryFrom<IBig> for $signed {
            type Error = OutOfRangeError;

            fn try_from(bi: IBig) -> Result<Self, Self::Error> {
                <$signed as TryFrom<_>>::try_from(&bi)
            }
        }

        impl TryFrom<IBig> for $unsigned {
            type Error = OutOfRangeError;

            fn try_from(bi: IBig) -> Result<Self, Self::Error> {
                <$unsigned as TryFrom<_>>::try_from(&bi)
            }
        }

        impl TryFrom<&IBig> for $signed {
            type Error = OutOfRangeError;

            fn try_from(bi: &IBig) -> Result<Self, Self::Error> {
                if bi > &IBig::from(Self::MAX) {
                    Err(OutOfRangeError::above())
                } else if bi < &IBig::from(Self::MIN) {
                    Err(OutOfRangeError::below())
                } else {
                    bi.with_slice(|s| arr_to_int(s))
                        .ok_or_else(|| OutOfRangeError::above())
                }
            }
        }

        impl TryFrom<&IBig> for $unsigned {
            type Error = OutOfRangeError;

            fn try_from(bi: &IBig) -> Result<Self, Self::Error> {
                if bi > &IBig::from(Self::MAX) {
                    Err(OutOfRangeError::above())
                } else if bi < &IBig::from(Self::MIN) {
                    Err(OutOfRangeError::below())
                } else {
                    bi.with_slice(|s| arr_to_int(s))
                        .ok_or_else(|| OutOfRangeError::above())
                }
            }
        }

        // Casts

        impl numeric_traits::cast::FromTruncating<IBig> for $unsigned {
            fn truncate_from(val: IBig) -> Self {
                val.with_slice(|s| arr_to_int(s))
                    .unwrap_or(<$unsigned>::MAX)
            }
        }

        impl numeric_traits::cast::FromTruncating<IBig> for $signed {
            fn truncate_from(val: IBig) -> Self {
                val.with_slice(|s| arr_to_int(s)).unwrap_or(<$signed>::MAX)
                    * if val.is_negative() { -1 } else { 1 }
            }
        }

        impl numeric_traits::cast::FromChecked<IBig> for $unsigned {
            fn from_checked(val: IBig) -> Option<Self> {
                val.try_into().ok()
            }
        }

        impl numeric_traits::cast::FromChecked<IBig> for $signed {
            fn from_checked(val: IBig) -> Option<Self> {
                val.try_into().ok()
            }
        }

        impl numeric_traits::cast::FromSaturating<IBig> for $unsigned {
            fn saturate_from(val: IBig) -> Self {
                match val.try_into() {
                    Ok(val) => val,
                    Err(OutOfRangeError(Side::Above)) => Self::MAX,
                    Err(OutOfRangeError(Side::Below)) => Self::MIN,
                }
            }
        }

        impl numeric_traits::cast::FromSaturating<IBig> for $signed {
            fn saturate_from(val: IBig) -> Self {
                match val.try_into() {
                    Ok(val) => val,
                    Err(OutOfRangeError(Side::Above)) => Self::MAX,
                    Err(OutOfRangeError(Side::Below)) => Self::MIN,
                }
            }
        }

        impl numeric_traits::cast::FromTruncating<$signed> for IBig {
            fn truncate_from(val: $signed) -> Self {
                IBig::from(val)
            }
        }

        impl numeric_traits::cast::FromTruncating<$unsigned> for IBig {
            fn truncate_from(val: $unsigned) -> Self {
                IBig::from(val)
            }
        }

        impl numeric_traits::cast::FromChecked<$signed> for IBig {
            fn from_checked(val: $signed) -> Option<Self> {
                Some(IBig::from(val))
            }
        }

        impl numeric_traits::cast::FromChecked<$unsigned> for IBig {
            fn from_checked(val: $unsigned) -> Option<Self> {
                Some(IBig::from(val))
            }
        }

        impl numeric_traits::cast::FromSaturating<$signed> for IBig {
            fn saturate_from(val: $signed) -> Self {
                IBig::from(val)
            }
        }

        impl numeric_traits::cast::FromSaturating<$unsigned> for IBig {
            fn saturate_from(val: $unsigned) -> Self {
                IBig::from(val)
            }
        }

        // Comparison

        impl PartialEq<$signed> for IBig {
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

        impl PartialEq<$unsigned> for IBig {
            fn eq(&self, other: &$unsigned) -> bool {
                self.with_slice(|this| {
                    let arr = int_to_arr::<_, _, { arr_size::<$unsigned>() }>(*other);
                    this == IntSlice::shrink(&arr as &[_])
                })
            }
        }

        impl PartialOrd<$signed> for IBig {
            fn partial_cmp(&self, other: &$signed) -> Option<Ordering> {
                Some(IBig::cmp(self, &IBig::from(*other)))
            }
        }

        impl PartialOrd<$unsigned> for IBig {
            fn partial_cmp(&self, other: &$unsigned) -> Option<Ordering> {
                Some(IBig::cmp(self, &IBig::from(*other)))
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
        impl core::ops::$trait<IBig> for IBig {
            type Output = IBig;

            fn $meth(self, rhs: IBig) -> Self::Output {
                <&IBig as core::ops::$trait<&IBig>>::$meth(&self, &rhs)
            }
        }

        impl core::ops::$trait<&IBig> for IBig {
            type Output = IBig;

            fn $meth(self, rhs: &IBig) -> Self::Output {
                <&IBig as core::ops::$trait<&IBig>>::$meth(&self, rhs)
            }
        }

        impl core::ops::$trait<IBig> for &IBig {
            type Output = IBig;

            fn $meth(self, rhs: IBig) -> Self::Output {
                <&IBig as core::ops::$trait<&IBig>>::$meth(self, &rhs)
            }
        }

        impl core::ops::$trait<&IBig> for &IBig {
            type Output = IBig;

            fn $meth($self, $rhs: &IBig) -> Self::Output $block
        }
    };
}
