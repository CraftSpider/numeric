use std::borrow::Borrow;
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::linked::UnsyncLinked;
use crate::into_owned::IntoOwned;

const CHUNK_SIZE: usize = 32;

enum Find<T> {
    Exists(T),
    Dead(T),
    None,
}

pub struct Interned<T> {
    refs: AtomicUsize,
    val: UnsafeCell<Option<T>>,
}

impl<T> Interned<T> {
    #[inline]
    fn new_uninit() -> Interned<T> {
        Interned { refs: AtomicUsize::new(0), val: UnsafeCell::new(None) }
    }

    #[inline]
    fn val_opt(&self) -> Option<&T> {
        // SAFETY: Only access val immutably except in `set_val` which is unsafe
        unsafe { (*self.val.get()).as_ref() }
    }

    #[inline]
    pub fn val(&self) -> &T {
        // SAFETY: Public method only exposed to user after value is guaranteed initialized
        //         by internal code.
        unsafe { self.val_opt().unwrap_unchecked() }
    }

    /// # SAFETY
    ///
    /// Caller must be the only one accessing the slot to call this method
    #[inline]
    unsafe fn set_val(&self, val: T) {
        (*self.val.get()) = Some(val);
    }
}

pub struct Interner<T> {
    inner: UnsyncLinked<[Interned<T>; CHUNK_SIZE]>,
}

impl<T> Interner<T>
where
    T: Clone + PartialEq,
{
    #[must_use]
    pub const fn new() -> Interner<T> {
        Interner { inner: UnsyncLinked::new() }
    }

    fn find<U>(list: &UnsyncLinked<[Interned<T>; CHUNK_SIZE]>, val: &U) -> Find<(usize, usize)>
    where
        U: ?Sized + PartialEq,
        T: Borrow<U>
    {
        for (idx, i) in list.iter().enumerate() {
            for (idx2, i) in i.iter().enumerate() {
                // This intentionally allows reviving dead slots - saves work if you're rapidly dropping
                // and creating references to a value
                let count = i.refs.load(Ordering::Acquire);
                if i.val_opt().map_or(false, |cur_val| val == cur_val.borrow() ) {
                    return Find::Exists((idx, idx2));
                } else if count == 0 {
                    return Find::Dead((idx, idx2));
                }
            }
        }
        Find::None
    }

    #[inline]
    fn incr_inner(interned: &Interned<T>) {
        let val = interned.refs.fetch_add(1, Ordering::AcqRel);
        debug_assert_ne!(
            val, usize::MAX,
            "Too many instances of a single value!"
        );
    }

    #[inline]
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

    pub fn add<U, V>(&self, val: U) -> usize
    where
        U: IntoOwned<T> + Borrow<V>,
        T: Borrow<V>,
        V: ?Sized + PartialEq,
    {
        let find = Self::find(&self.inner, val.borrow());
        match find {
            Find::Exists((loc1, loc2)) => {
                Self::incr_inner(&self.inner[loc1][loc2]);
                loc1 * 32 + loc2
            }
            Find::Dead((loc1, loc2)) => {
                Self::incr_inner(&self.inner[loc1][loc2]);
                // SAFETY: Slot is dead, we're making it live, we are the only ones with access
                unsafe { self.inner[loc1][loc2].set_val(val.into_owned()) };
                loc1 * 32 + loc2
            }
            Find::None => {
                let len = self.inner.push([(); CHUNK_SIZE].map(|_| Interned::new_uninit()));
                Self::incr_inner(&self.inner[len - 1][0]);
                // SAFETY: Slot is empty, we're making it live, we are the only ones with access
                unsafe { self.inner[len - 1][0].set_val(val.into_owned()) };
                (len - 1) * 32
            }
        }
    }

    pub fn try_get(&self, offset: usize) -> Option<&T> {
        let (idx1, idx2) = Self::offset_to_idx(offset);
        let slot = &self.inner[idx1][idx2];
        if slot.refs.load(Ordering::Relaxed) == 0 {
            None
        } else {
            Some(slot.val())
        }
    }

    pub fn get(&self, offset: usize) -> &T {
        self.try_get(offset)
            .expect("Expected valid offset")
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

    #[test]
    fn test_no_dead() {
        let interner = Interner::new();

        let pos1 = interner.add(-1);
        interner.decr(pos1);
        assert!(matches!(
            interner.try_get(pos1),
            None
        ));
    }
}

