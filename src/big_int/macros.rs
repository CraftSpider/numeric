
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
        impl From<$signed> for BigInt {
            fn from(val: $signed) -> Self {
                let neg = val.is_negative();
                BigInt::new_slice(&int_to_arr(val.abs() as $unsigned), neg)
            }
        }

        impl From<$unsigned> for BigInt {
            fn from(val: $unsigned) -> Self {
                BigInt::new_slice(&int_to_arr(val), false)
            }
        }

        impl TryFrom<&BigInt> for $signed {
            type Error = OutOfRangeError;

            fn try_from(bi: &BigInt) -> Result<Self, Self::Error> {
                if bi > &BigInt::from(Self::MAX) || bi < &BigInt::from(Self::MIN) {
                    Err(OutOfRangeError)
                } else {
                    bi.with_slice(|s| arr_to_int(s.inner())).ok_or(OutOfRangeError)
                }
            }
        }

        impl TryFrom<&BigInt> for $unsigned {
            type Error = OutOfRangeError;

            fn try_from(bi: &BigInt) -> Result<Self, Self::Error> {
                if bi > &BigInt::from(Self::MAX) || bi < &BigInt::from(Self::MIN) {
                    Err(OutOfRangeError)
                } else {
                    bi.with_slice(|s| arr_to_int(s.inner())).ok_or(OutOfRangeError)
                }
            }
        }

        impl PartialEq<$signed> for BigInt {
            fn eq(&self, other: &$signed) -> bool {
                if self.is_negative() != other.is_negative() {
                    return false;
                }
                let other = other.abs();

                self.with_slice(|this| {
                    this == BitSlice::new(int_to_arr(other as $unsigned))
                })
            }
        }

        impl PartialEq<$unsigned> for BigInt {
            fn eq(&self, other: &$unsigned) -> bool {
                self.with_slice(|this| {
                    this == BitSlice::new(int_to_arr(*other))
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

        impl_ops_for_int!($signed);
        impl_ops_for_int!($unsigned);
    }
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
