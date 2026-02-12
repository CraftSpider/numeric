use core::borrow::Borrow;
use core::cell::UnsafeCell;
use core::mem;
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::linked::UnsyncLinked;
use crate::static_assert;

const CHUNK_SIZE: usize = 32;

enum Find<T> {
    Exists(T),
    Dead(T),
    None,
}

struct Interned<T> {
    refs: AtomicUsize,
    val: UnsafeCell<Option<T>>,
}

impl<T> Interned<T> {
    #[inline]
    fn new_uninit() -> Interned<T> {
        Interned {
            refs: AtomicUsize::new(0),
            val: UnsafeCell::new(None),
        }
    }

    #[inline]
    fn val_opt(&self) -> Option<&T> {
        // SAFETY: Only access val immutably except in `set_val` which is unsafe
        unsafe { (*self.val.get()).as_ref() }
    }

    #[inline]
    fn val(&self) -> &T {
        // SAFETY: Method only used after value is guaranteed initialized by internal code.
        unsafe { self.val_opt().unwrap_unchecked() }
    }

    /// # SAFETY
    ///
    /// Caller must be the only one accessing the slot to call this method
    #[inline]
    unsafe fn set_val(&self, val: T) {
        *self.val.get() = Some(val);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct InternId(usize);

static_assert!(mem::size_of::<InternId>() == mem::size_of::<usize>());

impl InternId {
    pub fn from_usize(val: usize) -> InternId {
        InternId(val)
    }

    pub fn into_usize(self) -> usize {
        self.0
    }
}

/// An optimized container that supports cross-thread, lock-free-ish
pub struct Interner<T> {
    inner: UnsyncLinked<[Interned<T>; CHUNK_SIZE]>,
}

impl<T> Interner<T>
where
    T: Clone + PartialEq,
{
    #[must_use]
    pub const fn new() -> Interner<T> {
        Interner {
            inner: UnsyncLinked::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Interner<T> {
        let list = UnsyncLinked::new();
        for _ in 0..((capacity + CHUNK_SIZE - 1) / 32) {
            list.push([(); CHUNK_SIZE].map(|_| Interned::new_uninit()));
        }
        Interner { inner: list }
    }

    fn find<U>(list: &UnsyncLinked<[Interned<T>; CHUNK_SIZE]>, val: &U) -> Find<(usize, usize)>
    where
        U: ?Sized + PartialEq,
        T: Borrow<U>,
    {
        for (idx, i) in list.iter().enumerate() {
            for (idx2, i) in i.iter().enumerate() {
                // This intentionally allows reviving dead slots - saves work if you're rapidly
                // dropping and creating references to a value

                // We use 0xFFFFFFFF to indicate a value currently being watched by another thread
                // This is effectively locking, but it means we have a very small locking surface
                // (a single interned item at once). We also only need to hold that lock if we
                // intend to set the value.

                let count = loop {
                    let count = i.refs.swap(0xFFFFFFFF, Ordering::AcqRel);
                    if count == 0xFFFFFFFF {
                        continue;
                    } else {
                        break count;
                    }
                };
                if i.val_opt().is_some_and(|cur_val| val == cur_val.borrow()) {
                    i.refs.swap(count, Ordering::AcqRel);
                    return Find::Exists((idx, idx2));
                } else if count == 0 {
                    return Find::Dead((idx, idx2));
                }
                i.refs.swap(count, Ordering::AcqRel);
            }
        }
        Find::None
    }

    #[inline]
    fn incr_inner(interned: &Interned<T>) {
        let val = interned.refs.fetch_add(1, Ordering::AcqRel);
        debug_assert_ne!(val, usize::MAX - 1, "Too many instances of a single value!");
    }

    #[inline]
    fn decr_inner(interned: &Interned<T>) {
        let _ = interned
            .refs
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |val| {
                val.checked_sub(1)
            });
    }

    #[inline(always)]
    fn offset_to_idx(offset: InternId) -> (usize, usize) {
        (offset.0 / 32, offset.0 % 32)
    }

    /// Get or insert an item into the interner. Note that this takes `O(N)` time with respect
    /// to the number of items in the interner, so avoid calling it in a hot loop if possible.
    pub fn add<U, V>(&self, val: U) -> InternId
    where
        U: Into<T> + Borrow<V>,
        T: Borrow<V>,
        V: ?Sized + PartialEq,
    {
        let find = Self::find(&self.inner, val.borrow());
        InternId::from_usize(match find {
            Find::Exists((loc1, loc2)) => {
                Self::incr_inner(&self.inner[loc1][loc2]);
                loc1 * CHUNK_SIZE + loc2
            }
            Find::Dead((loc1, loc2)) => {
                let interned = &self.inner[loc1][loc2];
                // SAFETY: Slot is dead, we're making it live, we are the only ones with access
                unsafe { interned.set_val(val.into()) };
                interned.refs.store(1, Ordering::Release);
                loc1 * CHUNK_SIZE + loc2
            }
            Find::None => {
                let len = self
                    .inner
                    .push([(); CHUNK_SIZE].map(|_| Interned::new_uninit()));
                Self::incr_inner(&self.inner[len - 1][0]);
                // SAFETY: Slot is empty, we're making it live, we are the only ones with access
                unsafe { self.inner[len - 1][0].set_val(val.into()) };
                (len - 1) * CHUNK_SIZE
            }
        })
    }

    pub fn try_get(&self, offset: InternId) -> Option<&T> {
        let (idx1, idx2) = Self::offset_to_idx(offset);
        let slot = &self.inner[idx1][idx2];
        if slot.refs.load(Ordering::Relaxed) == 0 {
            None
        } else {
            Some(slot.val())
        }
    }

    pub fn get(&self, offset: InternId) -> &T {
        self.try_get(offset).expect("Expected valid offset")
    }

    pub fn incr(&self, offset: InternId) {
        let (idx1, idx2) = Self::offset_to_idx(offset);
        Self::incr_inner(&self.inner[idx1][idx2]);
    }

    pub fn decr(&self, offset: InternId) {
        let (idx1, idx2) = Self::offset_to_idx(offset);
        Self::decr_inner(&self.inner[idx1][idx2]);
    }

    #[allow(dead_code)]
    pub fn refcount(&self, offset: InternId) -> usize {
        let (idx1, idx2) = Self::offset_to_idx(offset);
        self.inner[idx1][idx2].refs.load(Ordering::Relaxed)
    }
}

impl<T: Clone + PartialEq> Default for Interner<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::run_threaded;

    #[test]
    fn test_multi_thread() {
        // Pre-allocate capacity, since otherwise adding blocks can race and add multiple. Not a
        // correctness issue, but makes this test harder.
        let interner = Interner::<usize>::with_capacity(10);

        run_threaded(
            move || interner,
            |interner, idx| {
                let pos = interner.add(idx % 10);
                assert!(pos.0 < 10, "pos too big: {}", pos.0);
            },
        );
    }

    #[test]
    fn test_add() {
        let interner = Interner::<i32>::new();

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
        let interner = Interner::<i32>::new();

        // Create value
        let pos1 = interner.add(0);
        assert_eq!(interner.refcount(pos1.clone()), 1);
        // Kill the location
        interner.decr(pos1.clone());
        assert_eq!(interner.refcount(pos1.clone()), 0);
        // Revive it
        let pos2 = interner.add(0);

        assert_eq!(pos1, pos2);
        assert_eq!(interner.refcount(pos2), 1);
    }

    #[test]
    fn test_no_dead() {
        let interner = Interner::<i32>::new();

        let pos1 = interner.add(-1);
        interner.decr(pos1.clone());
        assert!(interner.try_get(pos1).is_none());
    }
}
