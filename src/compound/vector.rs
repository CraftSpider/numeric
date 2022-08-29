use std::array;
use num_traits::{Num, Zero};

pub type Vec2<T> = Vector<T, 2>;
pub type Vec3<T> = Vector<T, 3>;
pub type Vec4<T> = Vector<T, 3>;

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

    pub const fn new_array(arr: [T; N]) -> Vector<T, N> {
        Vector(arr)
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Num,
{

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

    pub fn z(&self) -> &T {
        &self.0[2]
    }

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

        Vector::new_array([x1 - x2, y1 - y2, z1 - z2])
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
