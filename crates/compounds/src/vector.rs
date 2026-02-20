//! Fixed-size mathematical vectors, of arbitrary type and dimension.

#[cfg(feature = "alloc")]
use crate::bivec::BiVector;
use crate::matrix::Matrix;
use core::array;
use core::ops::{Index, IndexMut};
use numeric_static_iter::{IntoStaticIter, StaticIter};
use numeric_traits::class::{Numeric, Real, RealSigned};
use numeric_traits::identity::Zero;
use numeric_traits::ops::checked::{CheckedAdd, CheckedSub};

/// Two-dimensional vector type. See [`Vector`] for more info.
pub type Vec2<T> = Vector<T, 2>;
/// Three-dimensional vector type. See [`Vector`] for more info.
pub type Vec3<T> = Vector<T, 3>;
/// Four-dimensional vector type. See [`Vector`] for more info.
pub type Vec4<T> = Vector<T, 4>;

/// Fixed-size vector type. Implements many of the common vector math operations.
///
///
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vector<T, const N: usize>([T; N]);

impl<T, const N: usize> Vector<T, N> {
    /// Create a new vector from the given values.
    ///
    /// The values are assigned to the vector in-order, so `array[0]` becomes `x`, `array[1]`
    /// becomes y, and so on.
    pub const fn new(array: [T; N]) -> Vector<T, N> {
        Vector(array)
    }

    /// Create a new vector with all elements set to zero
    pub fn zeroed() -> Vector<T, N>
    where
        T: Zero,
    {
        Vector(array::from_fn(|_| T::zero()))
    }

    /// Create a new vector with all elements set to the provided value
    pub fn from_scalar(val: T) -> Vector<T, N>
    where
        T: Clone,
    {
        let arr = array::from_fn(|_| val.clone());
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
    /// Get the sum of all elements in the vector
    pub fn sum(self) -> T {
        self.0.into_static_iter().sum()
    }

    /// Get the product of all elements in the vector
    pub fn product(self) -> T {
        self.0.into_static_iter().product()
    }

    /// Get the dot product of two vectors.
    pub fn dot_product(lhs: Vector<T, N>, rhs: Vector<T, N>) -> T {
        Iterator::zip(lhs.0.into_iter(), rhs.0)
            .map(|(l, r)| l * r)
            .fold(T::zero(), |acc, val| acc + val)
    }

    /// Get the distance squared between two vectors.
    ///
    /// This can be significantly faster than the distance, since it doesn't perform a square root
    /// operation, but preserves some useful properties, such as ordering.
    pub fn distance_squared(lhs: Vector<T, N>, rhs: Vector<T, N>) -> T {
        let two = T::one() + T::one();

        Iterator::zip(lhs.0.into_iter(), rhs.0)
            .map(|(l, r)| (l - r).pow(two.clone()))
            .fold(T::zero(), |acc, val| acc + val)
    }

    /// Get the distance between two vectors.
    pub fn distance(lhs: Vector<T, N>, rhs: Vector<T, N>) -> T {
        Self::distance_squared(lhs, rhs).sqrt()
    }
}

impl<T: RealSigned, const N: usize> Vector<T, N> {
    /// Get the wedge product between two vectors, resulting in a [`BiVector`].
    ///
    /// This operation is related to [Exterior Algebra](https://en.wikipedia.org/wiki/Exterior_algebra).
    /// One common usage is in the creation of a [`Rotor`], the N-dimensional variant of a
    /// quaternion.
    #[cfg(feature = "alloc")]
    #[doc(alias = "exterior")]
    pub fn wedge(lhs: Vector<T, N>, rhs: Vector<T, N>) -> BiVector<T, N> {
        BiVector::new(lhs, rhs)
    }
}

impl<T> Vector<T, 2> {
    /// Create this 2D vector from an `x` and `y` value.
    #[inline]
    pub fn from_xy(x: T, y: T) -> Vector<T, 2> {
        Vector::new([x, y])
    }

    /// Get the `x` value
    #[inline(always)]
    pub const fn x(&self) -> &T {
        &self.0[0]
    }

    /// Get the `x` value mutably
    #[inline(always)]
    pub const fn x_mut(&mut self) -> &mut T {
        &mut self.0[0]
    }

    /// Set the `x` value
    #[inline(always)]
    pub fn set_x(&mut self, val: T) {
        self.0[0] = val;
    }

    /// Get the `y` value
    #[inline(always)]
    pub const fn y(&self) -> &T {
        &self.0[1]
    }

    /// Get the `y` value mutably
    #[inline(always)]
    pub const fn y_mut(&mut self) -> &mut T {
        &mut self.0[1]
    }

    /// Set the `y` value
    #[inline(always)]
    pub fn set_y(&mut self, val: T) {
        self.0[1] = val;
    }
}

impl<T> Vector<T, 3> {
    /// Create this 3D vector from an `x`, `y`, and `z` value.
    #[inline]
    pub fn from_xyz(x: T, y: T, z: T) -> Vector<T, 3> {
        Vector::new([x, y, z])
    }

    /// Get the `x` value
    #[inline(always)]
    pub const fn x(&self) -> &T {
        &self.0[0]
    }

    /// Get the `x` value mutably
    #[inline(always)]
    pub const fn x_mut(&mut self) -> &mut T {
        &mut self.0[0]
    }

    /// Set the `x` value
    #[inline(always)]
    pub fn set_x(&mut self, val: T) {
        self.0[0] = val;
    }

    /// Get the `y` value
    #[inline(always)]
    pub const fn y(&self) -> &T {
        &self.0[1]
    }

    /// Get the `y` value mutably
    #[inline(always)]
    pub const fn y_mut(&mut self) -> &mut T {
        &mut self.0[1]
    }

    /// Set the `y` value
    #[inline(always)]
    pub fn set_y(&mut self, val: T) {
        self.0[1] = val;
    }

    /// Get the `z` value
    #[inline(always)]
    pub const fn z(&self) -> &T {
        &self.0[2]
    }

    /// Get the `z` value mutably
    #[inline(always)]
    pub const fn z_mut(&mut self) -> &mut T {
        &mut self.0[2]
    }

    /// Set the `z` value
    #[inline(always)]
    pub fn set_z(&mut self, val: T) {
        self.0[2] = val;
    }

    /// Compute the cross product of two vectors. Only both valid and unique in 3D.
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
    /// Create this 4D vector from an `x`, `y`, `z`, and `w` value.
    #[inline]
    pub fn from_xyzw(x: T, y: T, z: T, w: T) -> Vector<T, 4> {
        Vector::new([x, y, z, w])
    }

    /// Get the `x` value
    #[inline(always)]
    pub const fn x(&self) -> &T {
        &self.0[0]
    }

    /// Get the `x` value mutably
    #[inline(always)]
    pub const fn x_mut(&mut self) -> &mut T {
        &mut self.0[0]
    }

    /// Set the `x` value
    #[inline(always)]
    pub fn set_x(&mut self, val: T) {
        self.0[0] = val;
    }

    /// Get the `y` value
    #[inline(always)]
    pub const fn y(&self) -> &T {
        &self.0[1]
    }

    /// Get the `y` value mutably
    #[inline(always)]
    pub const fn y_mut(&mut self) -> &mut T {
        &mut self.0[1]
    }

    /// Set the `y` value
    #[inline(always)]
    pub fn set_y(&mut self, val: T) {
        self.0[1] = val;
    }

    /// Get the `z` value
    #[inline(always)]
    pub const fn z(&self) -> &T {
        &self.0[2]
    }

    /// Get the `z` value mutably
    #[inline(always)]
    pub const fn z_mut(&mut self) -> &mut T {
        &mut self.0[2]
    }

    /// Set the `z` value
    #[inline(always)]
    pub fn set_z(&mut self, val: T) {
        self.0[2] = val;
    }

    /// Get the `w` value
    #[inline(always)]
    pub const fn w(&self) -> &T {
        &self.0[3]
    }

    /// Get the `w` value mutably
    #[inline(always)]
    pub const fn w_mut(&mut self) -> &mut T {
        &mut self.0[3]
    }

    /// Set the `w` value
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
ops_impl!(Rem, rem, %);

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
