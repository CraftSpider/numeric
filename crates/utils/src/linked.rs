use alloc::boxed::Box;
use core::marker::PhantomData;
use core::ops::Index;
use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};

pub struct Node<T> {
    next: AtomicPtr<Node<T>>,
    val: T,
}

impl<T> Node<T> {
    fn new(val: T) -> *mut Node<T> {
        Box::into_raw(Box::new(Node {
            next: AtomicPtr::default(),
            val,
        }))
    }

    /// Set the next node, without creating a mutable reference
    fn set_next(this: *mut Node<T>, new: *mut Node<T>) -> bool {
        let next = Self::get_next(this);
        next.compare_exchange(ptr::null_mut(), new, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }

    fn get_next<'a>(this: *mut Node<T>) -> &'a AtomicPtr<Node<T>> {
        // SAFETY: We only access `next` behind immutable reference after creation
        unsafe { &(*this).next }
    }

    /// Get the next node, without creating a reference
    fn get_next_opt<'a>(this: *mut Node<T>) -> Option<&'a AtomicPtr<Node<T>>> {
        let ptr = Self::get_next(this);
        if ptr.load(Ordering::Acquire).is_null() {
            None
        } else {
            Some(ptr)
        }
    }

    /// Create a reference to the value in this node
    ///
    /// # Safety
    ///
    /// Pointer must be valid, and no mutable references must exist
    fn get_val<'a>(this: *mut Node<T>) -> &'a T {
        // SAFETY: We only access `val` behind immutable reference after creation.
        unsafe { &(*this).val }
    }
}

pub struct UnsyncLinked<T> {
    head: AtomicPtr<Node<T>>,
}

impl<T> UnsyncLinked<T> {
    pub const fn new() -> UnsyncLinked<T> {
        UnsyncLinked {
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }

    fn tail(&self) -> (*mut Node<T>, usize) {
        let mut cur_node = self.head.load(Ordering::Relaxed);

        if cur_node.is_null() {
            return (cur_node, 0);
        }

        let mut len = 1;

        while let Some(new_node) = Node::get_next_opt(cur_node) {
            cur_node = new_node.load(Ordering::Relaxed);
            len += 1;
        }

        (cur_node, len)
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.tail().1
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Acquire).is_null()
    }

    /// Returns the new length of the list
    pub fn push(&self, val: T) -> usize {
        let new = Node::new(val);
        match self
            .head
            .compare_exchange(ptr::null_mut(), new, Ordering::AcqRel, Ordering::Acquire)
        {
            Ok(_) => 1,
            Err(_) => {
                let (mut tail, len) = self.tail();
                while !Node::set_next(tail, new) {
                    if let Some(next) = Node::get_next_opt(tail) {
                        tail = next.load(Ordering::Relaxed);
                    }
                }
                len + 1
            }
        }
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        let mut cur_node = self.head.load(Ordering::Acquire);
        if cur_node.is_null() {
            return None;
        }
        for _ in 0..idx {
            cur_node = Node::get_next_opt(cur_node)?.load(Ordering::Relaxed);
        }
        Some(Node::get_val(cur_node))
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            cur: &self.head,
            phantom: PhantomData,
        }
    }
}

impl<T> Index<usize> for UnsyncLinked<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
            .unwrap_or_else(|| panic!("Index {index} out of bounds"))
    }
}

impl<T> Drop for UnsyncLinked<T> {
    fn drop(&mut self) {
        let mut cur = self.head.load(Ordering::Relaxed);
        while !cur.is_null() {
            // SAFETY: Drop means unique access - it's sound to drop nodes or access them mutably
            //         since we're guaranteed to be the only code doing it
            cur = unsafe { Box::from_raw(cur) }.next.load(Ordering::Relaxed);
        }
    }
}

// SAFETY: UnsyncLinked uses atomic access internally
unsafe impl<T> Send for UnsyncLinked<T> {}
// SAFETY: UnsyncLinked uses atomic access internally
unsafe impl<T> Sync for UnsyncLinked<T> {}

pub struct Iter<'a, T> {
    cur: &'a AtomicPtr<Node<T>>,
    phantom: PhantomData<&'a ()>,
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: 'a,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let ptr = self.cur.load(Ordering::Acquire);
        if ptr.is_null() {
            None
        } else {
            let out = Node::get_val(ptr);
            self.cur = Node::get_next(ptr);
            Some(out)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{run_threaded, THREAD_COUNT};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn index_add_thread() {
        let list = run_threaded(UnsyncLinked::new, |list, i| {
            if i % 2 == 1 {
                list.push(i);
            } else {
                let mut tries = 0;
                loop {
                    if list.get((i / 2) as usize).is_some() {
                        break;
                    } else if tries > 100 {
                        panic!("{}", list.len())
                    }
                    tries += 1;
                    thread::sleep(Duration::from_millis(10));
                }
            }
        });
        assert_eq!(list.len(), THREAD_COUNT / 2);
    }

    #[test]
    fn iter_add_thread() {
        let list = run_threaded(UnsyncLinked::new, |list, i| {
            if i % 2 == 0 {
                list.push(i);
            } else {
                assert_eq!(list.iter().sum::<usize>() % 2, 0)
            }
        });
        assert_eq!(list.len(), THREAD_COUNT / 2);
    }

    #[test]
    fn add_thread() {
        let list = run_threaded(UnsyncLinked::new, |list, i| {
            list.push(i);
        });
        assert_eq!(list.len(), THREAD_COUNT);
    }
}
