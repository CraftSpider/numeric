use std::cell::UnsafeCell;
use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::RwLockReadGuard;

mod linked;

use linked::UnsyncLinked;

const CHUNK_SIZE: usize = 32;

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
    pub fn new() -> Interner<T> {
        Interner {
            inner: UnsyncLinked::new_with([(); CHUNK_SIZE].map(|_| Interned::new_uninit())),
        }
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

    pub fn add(&self, val: T) -> usize {
        let find = Self::find(&self.inner, &val);
        match find {
            Find::Exists((loc1, loc2)) => {
                Self::incr_inner(&self.inner[loc1][loc2]);
                loc1 * 32 + loc2
            }
            Find::Dead((loc1, loc2)) => {
                Self::incr_inner(&self.inner[loc1][loc2]);
                self.inner[loc1][loc2].set_val(val);
                loc1 * 32 + loc2
            }
            Find::None => {
                let len = self.inner.push([(); CHUNK_SIZE].map(|_| Interned::new_uninit()));
                Self::incr_inner(&self.inner[len - 1][0]);
                self.inner[len - 1][0].set_val(val);
                (len - 1) * 32
            }
        }
    }

    pub fn get(&self, offset: usize) -> &Interned<T> {
        &self.inner[offset / 32][offset % 32]
    }

    pub fn incr(&self, offset: usize) {
        Self::incr_inner(&self.inner[offset / 32][offset % 32])
    }

    pub fn decr(&self, offset: usize) {
        Self::decr_inner(&self.inner[offset / 32][offset % 32])
    }
}
