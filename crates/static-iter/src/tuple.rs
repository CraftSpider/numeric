//! Static length iterators over homogenous tuples

use crate::{FromStaticIter, IntoStaticIter, StaticIter};
use core::convert::Infallible;
use core::hint::unreachable_unchecked;
use core::mem::MaybeUninit;
use core::ops::ControlFlow;
use core::{mem, ptr};

macro_rules! tuple_impl {
    ($len:tt => $first_num:tt $first_ty:tt $($num:tt $ty:tt)*) => {
        impl<T> Sealed for ($first_ty, $($ty,)*) {}

        impl<T> MaybeTuple for ($first_ty, $($ty,)*) {
            type MaybeTuple = (MaybeUninit<$first_ty>, $(MaybeUninit<$ty>,)*);

            #[inline]
            fn convert(self) -> Self::MaybeTuple {
                (MaybeUninit::new(self.$first_num), $(MaybeUninit::new(self.$num),)*)
            }
        }

        impl<T> FromStaticIter<T, $len> for ($first_ty, $($ty,)*) {
            type Uninit = (MaybeUninit<$first_ty>, $(MaybeUninit<$ty>,)*);
            type Break = Infallible;

            fn uninit() -> Self::Uninit {
                (MaybeUninit::uninit(), $({ _ = $num; MaybeUninit::uninit() },)*)
            }

            fn write(mut this: Self::Uninit, idx: usize, val: T) -> ControlFlow<Self::Break, Self::Uninit> {
                match idx {
                    $first_num => this.$first_num.write(val),
                    $(
                        $num => this.$num.write(val),
                    )*
                    _ => unreachable!(),
                };
                ControlFlow::Continue(this)
            }

            unsafe fn finish(this: ControlFlow<Self::Break, Self::Uninit>) -> Self {
                let ControlFlow::Continue(this) = this;
                (this.$first_num.assume_init(), $(this.$num.assume_init(),)*)
            }
        }

        impl<T> IntoStaticIter<$len> for ($first_ty, $($ty,)*) {
            type Item = T;
            type Iter = IntoIter<Self>;

            fn into_static_iter(self) -> Self::Iter {
                IntoIter::new(self)
            }
        }

        impl<T> StaticIter<$len> for IntoIter<($first_ty, $($ty,)*)> {
            type Item = T;

            unsafe fn idx(&mut self, idx: usize) -> Self::Item {
                match idx {
                    $first_num => mem::replace(&mut self.tuple.$first_num, MaybeUninit::uninit()).assume_init(),
                    $(
                        $num => mem::replace(&mut self.tuple.$num, MaybeUninit::uninit()).assume_init(),
                    )*
                    // SAFETY: Safety requirement of the caller that idx in 0..N
                    _ => unsafe { unreachable_unchecked() },
                }
            }
        }

        impl<'a, T> IntoStaticIter<$len> for &'a ($first_ty, $($ty,)*) {
            type Item = &'a T;
            type Iter = RefIter<'a, ($first_ty, $($ty,)*)>;

            fn into_static_iter(self) -> Self::Iter {
                RefIter::new(self)
            }
        }

        impl<'a, T> StaticIter<$len> for RefIter<'a, ($first_ty, $($ty,)*)> {
            type Item = &'a T;

            unsafe fn idx(&mut self, idx: usize) -> Self::Item {
                match idx {
                    $first_num => &self.0.$first_num,
                    $(
                        $num => &self.0.$num,
                    )*
                    // SAFETY: Safety requirement of the caller that idx in 0..N
                    _ => unsafe { unreachable_unchecked() },
                }
            }
        }

        impl<'a, T> IntoStaticIter<$len> for &'a mut ($first_ty, $($ty,)*) {
            type Item = &'a mut T;
            type Iter = MutIter<'a, ($first_ty, $($ty,)*)>;

            fn into_static_iter(self) -> Self::Iter {
                MutIter::new(self)
            }
        }

        impl<'a, T> StaticIter<$len> for MutIter<'a, ($first_ty, $($ty,)*)> {
            type Item = &'a mut T;

            unsafe fn idx(&mut self, idx: usize) -> Self::Item {
                type Tuple<T> = ($first_ty, $($ty,)*);

                let ptr = ptr::from_mut(self.0);
                match idx {
                    $first_num => {
                        let offset = mem::offset_of!(Tuple<T>, $first_num);
                        &mut *ptr.byte_add(offset).cast::<T>()
                    }
                    $(
                        $num => {
                            let offset = mem::offset_of!(Tuple<T>, $num);
                            &mut *ptr.byte_add(offset).cast::<T>()
                        }
                    )*
                    // SAFETY: Safety requirement of the caller that idx in 0..N
                    _ => unsafe { unreachable_unchecked() },
                }
            }
        }

        tuple_impl!($first_num => $($num $ty)*);
    };
    (0 =>) => {}
}

tuple_impl!(12 => 11 T 10 T 9 T 8 T 7 T 6 T 5 T 4 T 3 T 2 T 1 T 0 T);

mod sealed {
    pub trait Sealed {}
}
use sealed::Sealed;

/// Internal trait for converting a tuple into a tuple of `MaybeUninit`
pub trait MaybeTuple: Sealed {
    /// Tuple of `MaybeUninit` values
    type MaybeTuple;

    /// Convert the tuple into a tuple of `MaybeUninit` values
    fn convert(self) -> Self::MaybeTuple;
}

/// Static iterator over owned values of a homogenous tuple
pub struct IntoIter<T: MaybeTuple> {
    tuple: T::MaybeTuple,
}

impl<T: MaybeTuple> IntoIter<T> {
    #[inline]
    fn new(tuple: T) -> Self {
        Self {
            tuple: tuple.convert(),
        }
    }
}

/// Static iterator over borrowed values of a homogenous tuple
pub struct RefIter<'a, T>(&'a T);

impl<'a, T> RefIter<'a, T> {
    #[inline]
    fn new(tuple: &'a T) -> Self {
        Self(tuple)
    }
}

/// Static iterator over mutably borrowed values of a homogenous tuple
pub struct MutIter<'a, T>(&'a mut T);

impl<'a, T> MutIter<'a, T> {
    #[inline]
    fn new(tuple: &'a mut T) -> Self {
        Self(tuple)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_iter() {
        let iter = (1, 2, 3, 4).into_static_iter();
        assert_eq!(iter.fold(0, |acc, val| acc + val), 10);
    }

    #[test]
    fn test_ref_iter() {
        let iter = (&(1, 2, 3, 4)).into_static_iter();
        assert_eq!(iter.fold(0, |acc, val| acc + *val), 10);
    }

    #[test]
    fn test_mut_iter() {
        let mut tuple = (1, 2, 3, 4);
        let iter = (&mut tuple).into_static_iter();
        iter.for_each(|val| *val += 1);
        assert_eq!(tuple, (2, 3, 4, 5))
    }

    #[test]
    fn test_collect() {
        let tuple = (1, 2, 3, 4).into_static_iter().collect::<(_, _, _, _)>();

        assert_eq!(tuple, (1, 2, 3, 4))
    }
}
