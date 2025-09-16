#[cfg(feature = "std")]
use crate::bivec::BiVector;
use crate::matrix::Matrix;
use core::array;
use core::ops::{Index, IndexMut};
use numeric_static_iter::{IntoStaticIter, StaticIter};
use numeric_traits::class::{Numeric, Real, RealSigned};
use numeric_traits::identity::Zero;
use numeric_traits::ops::checked::{CheckedAdd, CheckedSub};

pub type Vec2<T> = Vector<T, 2>;
pub type Vec3<T> = Vector<T, 3>;
pub type Vec4<T> = Vector<T, 4>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vector<T, const N: usize>([T; N]);

impl<T, const N: usize> Vector<T, N> {
    pub const fn new(array: [T; N]) -> Vector<T, N> {
        Vector(array)
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
        Vector::new(arr)
    }

    /// Convert this [`Vector`] into a single-row [`Matrix`]
    pub fn into_row(self) -> Matrix<T, 1, N> {
        Matrix::new([self.into()])
    }

    /// Convert this [`Vector`] into a single-column [`Matrix`]
    pub fn into_column(self) -> Matrix<T, N, 1> {
        let rows = <[_; N]>::from(self)
            .into_static_iter()
            .map(|i| [i])
            .collect();
        Matrix::new(rows)
    }
}

impl<T: Real, const N: usize> Vector<T, N> {
    pub fn sum(self) -> T {
        self.0.into_static_iter().sum()
    }

    pub fn product(self) -> T {
        self.0.into_static_iter().product()
    }

    pub fn dot_product(lhs: Vector<T, N>, rhs: Vector<T, N>) -> T {
        Iterator::zip(lhs.0.into_iter(), rhs.0)
            .map(|(l, r)| l * r)
            .fold(T::zero(), |acc, val| acc + val)
    }

    pub fn distance_squared(lhs: Vector<T, N>, rhs: Vector<T, N>) -> T {
        let two = T::one() + T::one();

        Iterator::zip(lhs.0.into_iter(), rhs.0)
            .map(|(l, r)| (l - r).pow(two.clone()))
            .fold(T::zero(), |acc, val| acc + val)
    }

    pub fn distance(lhs: Vector<T, N>, rhs: Vector<T, N>) -> T {
        Self::distance_squared(lhs, rhs).sqrt()
    }
}

impl<T: RealSigned, const N: usize> Vector<T, N> {
    #[cfg(feature = "std")]
    #[doc(alias = "exterior")]
    pub fn wedge(lhs: Vector<T, N>, rhs: Vector<T, N>) -> BiVector<T, N> {
        BiVector::new(lhs, rhs)
    }
}

impl<T> Vector<T, 2> {
    #[inline]
    pub fn from_xy(x: T, y: T) -> Vector<T, 2> {
        Vector::new([x, y])
    }

    #[inline(always)]
    pub const fn x(&self) -> &T {
        &self.0[0]
    }

    #[inline(always)]
    pub const fn x_mut(&mut self) -> &mut T {
        &mut self.0[0]
    }

    #[inline(always)]
    pub fn set_x(&mut self, val: T) {
        self.0[0] = val;
    }

    #[inline(always)]
    pub const fn y(&self) -> &T {
        &self.0[1]
    }

    #[inline(always)]
    pub const fn y_mut(&mut self) -> &mut T {
        &mut self.0[1]
    }

    #[inline(always)]
    pub fn set_y(&mut self, val: T) {
        self.0[1] = val;
    }
}

impl<T> Vector<T, 3> {
    #[inline]
    pub fn from_xyz(x: T, y: T, z: T) -> Vector<T, 3> {
        Vector::new([x, y, z])
    }

    #[inline(always)]
    pub const fn x(&self) -> &T {
        &self.0[0]
    }

    #[inline(always)]
    pub const fn x_mut(&mut self) -> &mut T {
        &mut self.0[0]
    }

    #[inline(always)]
    pub fn set_x(&mut self, val: T) {
        self.0[0] = val;
    }

    #[inline(always)]
    pub const fn y(&self) -> &T {
        &self.0[1]
    }

    #[inline(always)]
    pub const fn y_mut(&mut self) -> &mut T {
        &mut self.0[1]
    }

    #[inline(always)]
    pub fn set_y(&mut self, val: T) {
        self.0[1] = val;
    }

    #[inline(always)]
    pub const fn z(&self) -> &T {
        &self.0[2]
    }

    #[inline(always)]
    pub const fn z_mut(&mut self) -> &mut T {
        &mut self.0[2]
    }

    #[inline(always)]
    pub fn set_z(&mut self, val: T) {
        self.0[2] = val;
    }

    pub fn cross(self, other: Self) -> Vector<T, 3>
    where
        T: Numeric + Clone,
    {
        let x1 = self.y().clone() * other.z().clone();
        let x2 = self.z().clone() * other.y().clone();

        let y1 = self.z().clone() * other.x().clone();
        let y2 = self.x().clone() * other.z().clone();

        let z1 = self.x().clone() * other.y().clone();
        let z2 = self.y().clone() * other.x().clone();

        Vector::new([x1 - x2, y1 - y2, z1 - z2])
    }
}

impl<T> Vector<T, 4> {
    pub fn from_xyzw(x: T, y: T, z: T, w: T) -> Vector<T, 4> {
        Vector::new([x, y, z, w])
    }

    #[inline(always)]
    pub const fn x(&self) -> &T {
        &self.0[0]
    }

    #[inline(always)]
    pub const fn x_mut(&mut self) -> &mut T {
        &mut self.0[0]
    }

    #[inline(always)]
    pub fn set_x(&mut self, val: T) {
        self.0[0] = val;
    }

    #[inline(always)]
    pub const fn y(&self) -> &T {
        &self.0[1]
    }

    #[inline(always)]
    pub const fn y_mut(&mut self) -> &mut T {
        &mut self.0[1]
    }

    #[inline(always)]
    pub fn set_y(&mut self, val: T) {
        self.0[1] = val;
    }

    #[inline(always)]
    pub const fn z(&self) -> &T {
        &self.0[2]
    }

    #[inline(always)]
    pub const fn z_mut(&mut self) -> &mut T {
        &mut self.0[2]
    }

    #[inline(always)]
    pub fn set_z(&mut self, val: T) {
        self.0[2] = val;
    }

    #[inline(always)]
    pub const fn w(&self) -> &T {
        &self.0[3]
    }

    #[inline(always)]
    pub const fn w_mut(&mut self) -> &mut T {
        &mut self.0[3]
    }

    #[inline(always)]
    pub fn set_w(&mut self, val: T) {
        self.0[3] = val;
    }
}

impl<T: Zero, const N: usize> Zero for Vector<T, N> {
    fn zero() -> Self {
        Vector(array::from_fn(|_| T::zero()))
    }

    fn is_zero(&self) -> bool {
        (&self.0).into_static_iter().all(T::is_zero)
    }
}

impl<T: Default, const N: usize> Default for Vector<T, N> {
    fn default() -> Self {
        Vector(array::from_fn(|_| T::default()))
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(value: [T; N]) -> Self {
        Vector::new(value)
    }
}

impl<T, const N: usize> From<Vector<T, N>> for [T; N] {
    fn from(value: Vector<T, N>) -> Self {
        value.0
    }
}

impl<T, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
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
                let new = self.0.into_static_iter()
                    .zip(rhs.0.into_static_iter())
                    .map(|(l, r)| l $op r)
                    .collect();
                Vector(new)
            }
        }

        impl<'a, T, const N: usize> core::ops::$trait<&'a Vector<T, N>> for Vector<T, N>
        where
            T: core::ops::$trait<&'a T>,
        {
            type Output = Vector<T::Output, N>;

            fn $meth(self, rhs: &'a Self) -> Self::Output {
                let new = self.0.into_static_iter()
                    .zip((&rhs.0).into_static_iter())
                    .map(|(l, r)| l $op r)
                    .collect();
                Vector(new)
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

impl<T, const N: usize> CheckedAdd for Vector<T, N>
where
    T: CheckedAdd,
{
    type Output = Vector<T::Output, N>;

    fn checked_add(self, rhs: Self) -> Option<Self::Output> {
        let new = self
            .0
            .into_static_iter()
            .zip(rhs.0.into_static_iter())
            .map(|(l, r)| l.checked_add(r))
            .collect::<Option<_>>()?;
        Some(Vector(new))
    }
}

impl<T, const N: usize> CheckedSub for Vector<T, N>
where
    T: CheckedSub,
{
    type Output = Vector<T::Output, N>;

    fn checked_sub(self, rhs: Self) -> Option<Self::Output> {
        let new = self
            .0
            .into_static_iter()
            .zip(rhs.0.into_static_iter())
            .map(|(l, r)| l.checked_sub(r))
            .collect::<Option<_>>()?;
        Some(Vector(new))
    }
}
