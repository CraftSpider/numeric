macro_rules! impl_op {
    (add($self:ident: $ty:ty, $rhs:ident) => $block:block) => {
        impl_op!($ty, add, Add, $self, $rhs, $block);
    };
    (sub($self:ident: $ty:ty, $rhs:ident) => $block:block) => {
        impl_op!($ty, sub, Sub, $self, $rhs, $block);
    };
    (mul($self:ident: $ty:ty, $rhs:ident) => $block:block) => {
        impl_op!($ty, mul, Mul, $self, $rhs, $block);
    };
    (div($self:ident: $ty:ty, $rhs:ident) => $block:block) => {
        impl_op!($ty, div, Div, $self, $rhs, $block);
    };
    (rem($self:ident: $ty:ty, $rhs:ident) => $block:block) => {
        impl_op!($ty, rem, Rem, $self, $rhs, $block);
    };
    (shl($self:ident: $ty:ty, $rhs:ident) => $block:block) => {
        impl_op!($ty, shl, Shl, $self, $rhs, $block);
    };
    (shr($self:ident: $ty:ty, $rhs:ident) => $block:block) => {
        impl_op!($ty, shr, Shr, $self, $rhs, $block);
    };
    (bitand($self:ident: $ty:ty, $rhs:ident) => $block:block) => {
        impl_op!($ty, bitand, BitAnd, $self, $rhs, $block);
    };
    (bitor($self:ident: $ty:ty, $rhs:ident) => $block:block) => {
        impl_op!($ty, bitor, BitOr, $self, $rhs, $block);
    };
    (bitxor($self:ident: $ty:ty, $rhs:ident) => $block:block) => {
        impl_op!($ty, bitxor, BitXor, $self, $rhs, $block);
    };
    ($ty:ty, $meth:ident, $trait:ident, $self:ident, $rhs:ident, $block:block) => {
        impl core::ops::$trait<$ty> for $ty {
            type Output = $ty;

            fn $meth(self, rhs: $ty) -> Self::Output {
                <&$ty as core::ops::$trait<&$ty>>::$meth(&self, &rhs)
            }
        }

        impl core::ops::$trait<&$ty> for $ty {
            type Output = $ty;

            fn $meth(self, rhs: &$ty) -> Self::Output {
                <&$ty as core::ops::$trait<&$ty>>::$meth(&self, rhs)
            }
        }

        impl core::ops::$trait<$ty> for &$ty {
            type Output = $ty;

            fn $meth(self, rhs: $ty) -> Self::Output {
                <&$ty as core::ops::$trait<&$ty>>::$meth(self, &rhs)
            }
        }

        impl core::ops::$trait<&$ty> for &$ty {
            type Output = $ty;

            fn $meth($self, $rhs: &$ty) -> Self::Output $block
        }
    };
}

macro_rules! impl_assign_op {
    (add($self:ident: $ty:ty, $rhs:ident) => $block:block) => {
        impl_assign_op!($ty, add_assign, AddAssign, $self, $rhs, $block);
    };
    (sub($self:ident: $ty:ty, $rhs:ident) => $block:block) => {
        impl_assign_op!($ty, sub_assign, SubAssign, $self, $rhs, $block);
    };
    (mul($self:ident: $ty:ty, $rhs:ident) => $block:block) => {
        impl_assign_op!($ty, mul_assign, MulAssign, $self, $rhs, $block);
    };
    (div($self:ident: $ty:ty, $rhs:ident) => $block:block) => {
        impl_assign_op!($ty, div_assign, DivAssign, $self, $rhs, $block);
    };
    (rem($self:ident: $ty:ty, $rhs:ident) => $block:block) => {
        impl_assign_op!($ty, rem_assign, RemAssign, $self, $rhs, $block);
    };
    (shl($self:ident: $ty:ty, $rhs:ident) => $block:block) => {
        impl_assign_op!($ty, shl_assign, ShlAssign, $self, $rhs, $block);
    };
    (shr($self:ident: $ty:ty, $rhs:ident) => $block:block) => {
        impl_assign_op!($ty, shr_assign, ShrAssign, $self, $rhs, $block);
    };
    (bitand($self:ident: $ty:ty, $rhs:ident) => $block:block) => {
        impl_assign_op!($ty, bitand_assign, BitAndAssign, $self, $rhs, $block);
    };
    (bitor($self:ident: $ty:ty, $rhs:ident) => $block:block) => {
        impl_assign_op!($ty, bitor_assign, BitOrAssign, $self, $rhs, $block);
    };
    (bitxor($self:ident: $ty:ty, $rhs:ident) => $block:block) => {
        impl_assign_op!($ty, bitxor_assign, BitXorAssign, $self, $rhs, $block);
    };
    ($ty:ty, $meth:ident, $trait:ident, $self:ident, $rhs:ident, $block:block) => {
        impl core::ops::$trait<$ty> for $ty {
            fn $meth(&mut self, rhs: $ty) {
                <$ty as core::ops::$trait<&$ty>>::$meth(self, &rhs)
            }
        }

        impl core::ops::$trait<&$ty> for $ty {
            fn $meth(&mut $self, $rhs: &$ty) $block
        }
    };
}
