use std::mem::MaybeUninit;
use crate::static_iter::adapter::{Enumerate, Map, Zip};

struct DropGuard<F: FnOnce()> {
    on_drop: Option<F>,
}

impl<F: FnOnce()> DropGuard<F> {
    fn new(on_drop: F) -> DropGuard<F> {
        DropGuard { on_drop: Some(on_drop) }
    }
}

impl<F> Drop for DropGuard<F>
where
    F: FnOnce(),
{
    fn drop(&mut self) {
        (self.on_drop.take().unwrap())()
    }
}

pub trait StaticCollect<T, const N: usize> {
    type Uninit;

    fn uninit() -> Self::Uninit;
    fn write(uninit: &mut Self::Uninit, idx: usize, val: T);

    /// # Safety
    ///
    /// `write` mut have been called for all values from `0` to `init` at least once
    unsafe fn drop(uninit: Self::Uninit, init: usize);

    /// # Safety
    ///
    /// `write` must have been called for all values from `0` to `N` at least once
    unsafe fn assume_init(uninit: Self::Uninit) -> Self;
}

impl<T, const N: usize> StaticCollect<T, N> for [T; N] {
    type Uninit = [MaybeUninit<T>; N];

    #[inline]
    fn uninit() -> Self::Uninit {
        // SAFETY: `[MaybeUninit<T>; N]` is uninhabited
        unsafe { MaybeUninit::uninit().assume_init() }
    }

    #[inline]
    fn write(uninit: &mut Self::Uninit, idx: usize, val: T) {
        uninit[idx].write(val);
    }

    #[inline]
    unsafe fn drop(mut uninit: Self::Uninit, init: usize) {
        for val in uninit.iter_mut().take(init) {
            // SAFETY: All values `0..init` are guaranteed initialized
            unsafe { val.assume_init_drop() };
        }
    }

    #[inline]
    unsafe fn assume_init(uninit: Self::Uninit) -> Self {
        // SAFETY: Per contract of this function, `write_next` has been called to initialize all values
        let out = unsafe { (&uninit as *const [_; N]).cast::<[T; N]>().read() };
        core::mem::forget(uninit);
        out
    }
}

pub trait StaticIter<const N: usize>: Sized {
    type Item;

    /// # Safety
    ///
    /// This function must be called at most once for each index in order starting from zero
    /// The index must be in the range 0..N
    unsafe fn idx(&mut self, idx: usize) -> Self::Item;

    #[inline]
    fn map<T, F: FnMut(Self::Item) -> T>(self, func: F) -> Map<Self, F> {
        Map { inner: self, func }
    }

    #[inline]
    fn zip<I>(self, other: I) -> Zip<Self, I>
    where
        I: StaticIter<N>,
    {
        Zip { left: self, right: other }
    }

    #[inline]
    fn enumerate(self) -> Enumerate<Self> {
        Enumerate { inner: self }
    }

    #[inline]
    fn fold<T, F: FnMut(T, Self::Item) -> T>(mut self, mut start: T, mut func: F) -> T {
        for idx in 0..N {
            // SAFETY: Follows contract of `idx` - we call exactly once for each value from `0..N`
            let item = unsafe { self.idx(idx) };
            start = func(start, item);
        }
        start
    }

    // TODO: This really wants to use `Try`
    #[inline]
    fn try_fold<T, E, F: FnMut(T, Self::Item) -> Result<T, E>>(mut self, mut start: T, mut func: F) -> Result<T, E> {
        for idx in 0..N {
            // SAFETY: Follows contract of `idx` - we call exactly once for each value from `0..N`
            let item = unsafe { self.idx(idx) };
            start = func(start, item)?;
        }
        Ok(start)
    }

    fn collect<C: StaticCollect<Self::Item, N>>(self) -> C {
        let out = self.enumerate().fold(
            C::uninit(),
            |mut out, (idx, val)| {
                C::write(&mut out, idx, val);
                out
            });
        // SAFETY: After the fold call, all values from 0..N will have been written
        unsafe { C::assume_init(out) }
    }

    fn any<F: FnMut(Self::Item) -> bool>(self, mut func: F) -> bool {
        self.try_fold((), |(), x| {
            if func(x) { Err(()) } else { Ok(()) }
        }) == Err(())
    }

    fn all<F: FnMut(Self::Item) -> bool>(self, mut func: F) -> bool {
        self.try_fold((), |(), x| {
            if func(x) { Ok(()) } else { Err(()) }
        }) == Ok(())
    }
}

pub trait IntoStaticIter<const N: usize> {
    type Item;
    type Iter: StaticIter<N, Item = Self::Item>;

    fn into_static_iter(self) -> Self::Iter;
}

pub mod adapter;
pub mod array;
