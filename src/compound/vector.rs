use core::array;
use num_traits::{Num, Zero};
use crate::traits::Real;

pub type Vec2<T> = Vector<T, 2>;
pub type Vec3<T> = Vector<T, 3>;
pub type Vec4<T> = Vector<T, 4>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T, const N: usize>([T; N]);

impl<T, const N: usize> Vector<T, N> {
    pub fn new() -> Vector<T, N>
    where
        T: Default,
    {
        Self::default()
    }

    pub fn zeroed() -> Vector<T, N>
    where
        T: Zero,
    {
        Vector(array::from_fn(|_| T::zero()))
    }

    pub fn from_scalar(val: T) -> Vector<T, N>
    where
        T: Clone,
    {
        let arr = core::array::from_fn(|_| val.clone());
        Vector::from_array(arr)
    }

    pub const fn from_array(arr: [T; N]) -> Vector<T, N> {
        Vector(arr)
    }
}

impl<T: Real, const N: usize> Vector<T, N> {
    pub fn dot_product(lhs: Vector<T, N>, rhs: Vector<T, N>) -> T {
        Iterator::zip(lhs.0.into_iter(), rhs.0.into_iter())
            .map(|(l, r)| l * r)
            .fold(T::zero(), |acc, val| acc + val)
    }

    pub fn distance_squared(lhs: Vector<T, N>, rhs: Vector<T, N>) -> T {
        let two = T::one() + T::one();

        Iterator::zip(lhs.0.into_iter(), rhs.0.into_iter())
            .map(|(l, r)| (l - r).pow(two.clone()))
            .fold(T::zero(), |acc, val| acc + val)
    }

    pub fn distance(lhs: Vector<T, N>, rhs: Vector<T, N>) -> T {
        Self::distance_squared(lhs, rhs).sqrt()
    }
}

impl<T> Vector<T, 2> {
    #[inline(always)]
    pub fn x(&self) -> &T {
        &self.0[0]
    }

    #[inline(always)]
    pub fn set_x(&mut self, val: T) {
        self.0[0] = val;
    }

    #[inline(always)]
    pub fn y(&self) -> &T {
        &self.0[1]
    }

    #[inline(always)]
    pub fn set_y(&mut self, val: T) {
        self.0[1] = val;
    }
}

impl<T> Vector<T, 3> {
    #[inline(always)]
    pub fn x(&self) -> &T {
        &self.0[0]
    }

    #[inline(always)]
    pub fn set_x(&mut self, val: T) {
        self.0[0] = val;
    }

    #[inline(always)]
    pub fn y(&self) -> &T {
        &self.0[1]
    }

    #[inline(always)]
    pub fn set_y(&mut self, val: T) {
        self.0[1] = val;
    }

    #[inline(always)]
    pub fn z(&self) -> &T {
        &self.0[2]
    }

    #[inline(always)]
    pub fn set_z(&mut self, val: T) {
        self.0[2] = val;
    }

    pub fn cross(self, other: Self) -> Vector<T, 3>
    where
        T: Num + Clone,
    {
        let x1 = self.y().clone() * other.z().clone();
        let x2 = self.z().clone() * other.y().clone();

        let y1 = self.z().clone() * other.x().clone();
        let y2 = self.x().clone() * other.z().clone();

        let z1 = self.x().clone() * other.y().clone();
        let z2 = self.y().clone() * other.x().clone();

        Vector::from_array([x1 - x2, y1 - y2, z1 - z2])
    }
}

impl<T> Vector<T, 4> {
    #[inline(always)]
    pub fn x(&self) -> &T {
        &self.0[0]
    }

    #[inline(always)]
    pub fn set_x(&mut self, val: T) {
        self.0[0] = val;
    }

    #[inline(always)]
    pub fn y(&self) -> &T {
        &self.0[1]
    }

    #[inline(always)]
    pub fn set_y(&mut self, val: T) {
        self.0[1] = val;
    }

    #[inline(always)]
    pub fn z(&self) -> &T {
        &self.0[2]
    }

    #[inline(always)]
    pub fn set_z(&mut self, val: T) {
        self.0[2] = val;
    }

    #[inline(always)]
    pub fn w(&self) -> &T {
        &self.0[3]
    }

    #[inline(always)]
    pub fn set_w(&mut self, val: T) {
        self.0[3] = val;
    }
}

impl<T: Default, const N: usize> Default for Vector<T, N> {
    fn default() -> Self {
        Vector(array::from_fn(|_| T::default()))
    }
}

macro_rules! ops_impl {
    ($trait:ident, $meth:ident, $op:tt) => {
        impl<T, const N: usize> core::ops::$trait<Vector<T, N>> for Vector<T, N>
        where
            T: core::ops::$trait<T>,
        {
            type Output = Vector<T::Output, N>;

            fn $meth(self, rhs: Self) -> Self::Output {
                Vector(self.0.zip(rhs.0).map(|(a, b)| a $op b))
            }
        }

        impl<'a, T, const N: usize> core::ops::$trait<&'a Vector<T, N>> for Vector<T, N>
        where
            T: core::ops::$trait<&'a T>,
        {
            type Output = Vector<T::Output, N>;

            fn $meth(self, rhs: &'a Self) -> Self::Output {
                Vector(self.0.zip(rhs.0.each_ref()).map(|(a, b)| a $op b))
            }
        }

        impl<T, const N: usize> core::ops::$trait<T> for Vector<T, N>
        where
            T: core::ops::$trait<T> + Clone,
        {
            type Output = Vector<T::Output, N>;

            fn $meth(self, rhs: T) -> Self::Output {
                Vector(self.0.map(|a| a $op rhs.clone()))
            }
        }

        impl<'a, T, const N: usize> core::ops::$trait<&'a T> for Vector<T, N>
        where
            T: core::ops::$trait<&'a T>,
        {
            type Output = Vector<T::Output, N>;

            fn $meth(self, rhs: &'a T) -> Self::Output {
                Vector(self.0.map(|a| a $op rhs))
            }
        }
    };
}

ops_impl!(Add, add, +);
ops_impl!(Sub, sub, -);
ops_impl!(Mul, mul, *);
ops_impl!(Div, div, /);

macro_rules! assign_ops_impl {
    ($trait:ident, $meth:ident, $op:tt) => {
        impl<T, const N: usize> core::ops::$trait<Vector<T, N>> for Vector<T, N>
        where
            T: core::ops::$trait<T>,
        {
            fn $meth(&mut self, rhs: Vector<T, N>) {
                Iterator::zip(
                    self.0.iter_mut(),
                    rhs.0.into_iter(),
                )
                    .for_each(|(l, r)| *l $op r)
            }
        }

        impl<T, const N: usize> core::ops::$trait<T> for Vector<T, N>
        where
            T: core::ops::$trait<T> + Clone,
        {
            fn $meth(&mut self, rhs: T) {
                self.0.iter_mut()
                    .for_each(|l| *l $op rhs.clone())
            }
        }

        impl<'a, T, const N: usize> core::ops::$trait<&'a Vector<T, N>> for Vector<T, N>
        where
            T: core::ops::$trait<&'a T>,
        {
            fn $meth(&mut self, rhs: &'a Vector<T, N>) {
                Iterator::zip(
                    self.0.iter_mut(),
                    rhs.0.iter(),
                )
                    .for_each(|(l, r)| *l $op r)
            }
        }

        impl<'a, T, const N: usize> core::ops::$trait<&'a T> for Vector<T, N>
        where
            T: core::ops::$trait<&'a T>,
        {
            fn $meth(&mut self, rhs: &'a T) {
                self.0.iter_mut()
                    .for_each(|l| *l $op rhs)
            }
        }
    };
}

assign_ops_impl!(AddAssign, add_assign, +=);
assign_ops_impl!(SubAssign, sub_assign, -=);
assign_ops_impl!(MulAssign, mul_assign, *=);
assign_ops_impl!(DivAssign, div_assign, /=);
assign_ops_impl!(RemAssign, rem_assign, %=);
