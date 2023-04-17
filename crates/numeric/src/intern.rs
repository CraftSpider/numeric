use std::cell::UnsafeCell;
use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::RwLockReadGuard;

mod linked;

use linked::UnsyncLinked;

const CHUNK_SIZE: usize = 32;

pub trait IntoOwned<T> {
    fn into_owned(self) -> T;
}

impl<T> IntoOwned<T> for T {
    fn into_owned(self) -> T {
        self
    }
}

impl<T> IntoOwned<Vec<T>> for &[T]
where
    T: Clone,
{
    fn into_owned(self) -> Vec<T> {
        self.to_owned()
    }
}

impl<T> IntoOwned<Box<[T]>> for &[T]
where
    T: Copy,
{
    fn into_owned(self) -> Box<[T]> {
        self.into()
    }
}

pub struct SliceHack<'a, T>(pub &'a [T]);

impl<T> IntoOwned<Box<[T]>> for SliceHack<'_, T>
    where
        T: Copy,
{
    fn into_owned(self) -> Box<[T]> {
        self.0.into()
    }
}

impl<T> PartialEq<Box<[T]>> for SliceHack<'_, T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Box<[T]>) -> bool {
        self.0 == &**other
    }
}

pub enum Find<T> {
    Exists(T),
    Dead(T),
    None,
}

pub struct InternRef<'a, T> {
    guard: RwLockReadGuard<'a, Vec<Interned<T>>>,
    offset: usize,
}

impl<T> Deref for InternRef<'_, T> {
    type Target = Interned<T>;

    fn deref(&self) -> &Self::Target {
        &self.guard[self.offset]
    }
}

pub struct Interned<T> {
    refs: AtomicUsize,
    val: UnsafeCell<Option<T>>,
}

impl<T> Interned<T> {
    fn new_uninit() -> Interned<T> {
        Interned { refs: AtomicUsize::new(0), val: UnsafeCell::new(None) }
    }

    fn val_opt(&self) -> Option<&T> {
        unsafe { (*self.val.get()).as_ref() }
    }

    pub fn val(&self) -> &T {
        unsafe { (*self.val.get()).as_ref().unwrap_unchecked() }
    }

    fn set_val(&self, val: T) {
        unsafe { (*self.val.get()) = Some(val) };
    }
}

pub struct Interner<T> {
    inner: UnsyncLinked<[Interned<T>; CHUNK_SIZE]>,
}

impl<T> Interner<T>
where
    T: Clone + PartialEq,
{
    pub const fn new() -> Interner<T> {
        Interner { inner: UnsyncLinked::new() }
    }

    fn find<U>(list: &UnsyncLinked<[Interned<T>; CHUNK_SIZE]>, val: &U) -> Find<(usize, usize)>
    where
        U: PartialEq<T>,
    {
        for (idx, i) in list.iter().enumerate() {
            for (idx2, i) in i.iter().enumerate() {
                // This intentionally allows reviving dead slots - saves work if you're rapidly dropping
                // and creating references to a value
                let count = i.refs.load(Ordering::Acquire);
                if i.val_opt().map_or(false, |cur_val| val == cur_val) {
                    return Find::Exists((idx, idx2));
                } else if count == 0 {
                    return Find::Dead((idx, idx2));
                }
            }
        }
        Find::None
    }

    fn incr_inner(interned: &Interned<T>) {
        let val = interned.refs.fetch_add(1, Ordering::AcqRel);
        if val == usize::MAX {
            panic!("Too many instance of a single value!");
        }
    }

    fn decr_inner(interned: &Interned<T>) {
        let _ = interned.refs.fetch_update(
            Ordering::AcqRel,
            Ordering::Acquire,
            |val| val.checked_sub(1)
        );
    }

    #[inline(always)]
    fn offset_to_idx(offset: usize) -> (usize, usize) {
        (offset / 32, offset % 32)
    }

    pub fn add<U>(&self, val: U) -> usize
    where
        U: IntoOwned<T> + PartialEq<T>,
    {
        let find = Self::find(&self.inner, &val);
        match find {
            Find::Exists((loc1, loc2)) => {
                Self::incr_inner(&self.inner[loc1][loc2]);
                loc1 * 32 + loc2
            }
            Find::Dead((loc1, loc2)) => {
                Self::incr_inner(&self.inner[loc1][loc2]);
                self.inner[loc1][loc2].set_val(val.into_owned());
                loc1 * 32 + loc2
            }
            Find::None => {
                let len = self.inner.push([(); CHUNK_SIZE].map(|_| Interned::new_uninit()));
                Self::incr_inner(&self.inner[len - 1][0]);
                self.inner[len - 1][0].set_val(val.into_owned());
                (len - 1) * 32
            }
        }
    }

    pub fn get(&self, offset: usize) -> &Interned<T> {
        let (idx1, idx2) = Self::offset_to_idx(offset);
        &self.inner[idx1][idx2]
    }

    pub fn incr(&self, offset: usize) {
        let (idx1, idx2) = Self::offset_to_idx(offset);
        Self::incr_inner(&self.inner[idx1][idx2])
    }

    pub fn decr(&self, offset: usize) {
        let (idx1, idx2) = Self::offset_to_idx(offset);
        Self::decr_inner(&self.inner[idx1][idx2])
    }

    #[allow(dead_code)]
    pub fn refcount(&self, offset: usize) -> usize {
        let (idx1, idx2) = Self::offset_to_idx(offset);
        self.inner[idx1][idx2].refs.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let interner = Interner::new();

        let pos1 = interner.add(0);
        let pos2 = interner.add(0);
        let pos3 = interner.add(1);
        let pos4 = interner.add(1);

        assert_eq!(pos1, pos2);
        assert_eq!(pos3, pos4);
        assert_ne!(pos1, pos3);
    }

    #[test]
    fn test_dead_live() {
        let interner = Interner::new();

        // Create value
        let pos1 = interner.add(0);
        assert_eq!(interner.refcount(pos1), 1);
        // Kill the location
        interner.decr(pos1);
        assert_eq!(interner.refcount(pos1), 0);
        // Revive it
        let pos2 = interner.add(0);

        assert_eq!(pos1, pos2);
        assert_eq!(interner.refcount(pos2), 1);
    }
}
