//! Simple interner used by the default big integer implementation in `numeric-ints`.

use core::borrow::Borrow;
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::linked::UnsyncLinked;

const CHUNK_SIZE: usize = 32;

enum Find<T> {
    Exists(T),
    Dead(T),
    None,
}

/// An interned value. Must be dereferenced through the original `Interner` instance currently.
pub struct Interned<T> {
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

    /// Increment the reference count of an interned value.
    #[inline]
    pub fn incr(&self) {
        let val = self.refs.fetch_add(1, Ordering::AcqRel);
        debug_assert_ne!(val, usize::MAX - 1, "Too many instances of a single value!");
    }

    /// Decrement the reference count of an interned value.
    #[inline]
    pub fn decr(&self) {
        let _ = self
            .refs
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |val| {
                val.checked_sub(1)
            });
    }

    /// Attempt to get a reference to the value. Returns `None` if the value is dead (refcount is
    /// zero).
    pub fn try_get(&self) -> Option<&T> {
        if self.refs.load(Ordering::Relaxed) == 0 {
            None
        } else {
            Some(self.val())
        }
    }

    /// Get a reference to the value.
    ///
    /// # Panics
    ///
    /// If the value is dead (refcount is zero).
    pub fn get(&self) -> &T {
        if self.refs.load(Ordering::Relaxed) == 0 {
            panic!("Attempted to get value of dead interned value");
        } else {
            self.val()
        }
    }
}

/// An optimized container that supports cross-thread, lock-free-ish behavior. Users are responsible
/// for reference counting currently, as an implementation choice.
pub struct Interner<T> {
    inner: UnsyncLinked<[Interned<T>; CHUNK_SIZE]>,
}

impl<T> Interner<T>
where
    T: PartialEq,
{
    /// Create a new interner.
    #[must_use]
    pub const fn new() -> Interner<T> {
        Interner {
            inner: UnsyncLinked::new(),
        }
    }

    /// Create a new interner with a given capacity pre-allocated.
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
                    let count = i.refs.swap(0xFFFF_FFFF, Ordering::AcqRel);
                    if count == 0xFFFF_FFFF {
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

    /// Get or insert an item into the interner. Note that this takes `O(N)` time with respect
    /// to the number of items in the interner, so avoid calling it in a hot loop if possible.
    pub fn add<U, V>(&self, val: U) -> &Interned<T>
    where
        U: Into<T> + Borrow<V>,
        T: Borrow<V>,
        V: ?Sized + PartialEq,
    {
        let find = Self::find(&self.inner, val.borrow());
        match find {
            Find::Exists((loc1, loc2)) => {
                let interned = &self.inner[loc1][loc2];
                interned.incr();
                interned
            }
            Find::Dead((loc1, loc2)) => {
                let interned = &self.inner[loc1][loc2];
                // SAFETY: Slot is dead, we're making it live, we are the only ones with access
                unsafe { interned.set_val(val.into()) };
                interned.refs.store(1, Ordering::Release);
                interned
            }
            Find::None => {
                let len = self
                    .inner
                    .push([(); CHUNK_SIZE].map(|_| Interned::new_uninit()));
                let interned = &self.inner[len - 1][0];
                // SAFETY: Slot is empty, we're making it live, we are the only ones with access
                unsafe { interned.set_val(val.into()) };
                interned.incr();
                interned
            }
        }
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
    use core::ptr;

    #[test]
    fn test_multi_thread() {
        // Pre-allocate capacity, since otherwise adding blocks can race and add multiple. Not a
        // correctness issue, but makes this test harder.
        let interner = Interner::<usize>::with_capacity(10);

        run_threaded(
            move || interner,
            |interner, idx| {
                let _ = interner.add(idx % 10);
                assert_eq!(interner.inner.len(), 1, "interner over-allocated");
            },
        );
    }

    #[test]
    fn test_add() {
        let interner = Interner::<i32>::new();

        let val1 = interner.add(0);
        let val2 = interner.add(0);
        let val3 = interner.add(1);
        let val4 = interner.add(1);

        assert!(ptr::addr_eq(val1, val2));
        assert!(ptr::addr_eq(val3, val4));
        assert!(!ptr::addr_eq(val1, val3));
    }

    #[test]
    fn test_dead_live() {
        let interner = Interner::<i32>::new();

        // Create value
        let val1 = interner.add(0);
        assert_eq!(val1.refs.load(Ordering::Relaxed), 1);
        // Kill the location
        val1.decr();
        assert_eq!(val1.refs.load(Ordering::Relaxed), 0);
        // Revive it
        let val2 = interner.add(0);

        assert!(ptr::addr_eq(val1, val2));
        assert_eq!(val2.refs.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn test_no_dead() {
        let interner = Interner::<i32>::new();

        let val1 = interner.add(-1);
        val1.decr();
        assert!(val1.try_get().is_none());
    }
}
