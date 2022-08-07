use std::marker::PhantomData;
use std::ops::Index;
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

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
        let next = unsafe { &(*this).next };
        next.compare_exchange(ptr::null_mut(), new, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }

    fn get_next<'a>(this: *mut Node<T>) -> &'a AtomicPtr<Node<T>> {
        unsafe { &(*this).next }
    }

    /// Get the next node, without creating a reference
    fn get_next_opt<'a>(this: *mut Node<T>) -> Option<&'a AtomicPtr<Node<T>>> {
        let ptr = unsafe { &(*this).next };
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
    unsafe fn get_val<'a>(this: *mut Node<T>) -> &'a T {
        &(*this).val
    }
}

pub struct UnsyncLinked<T> {
    // TODO: AtomicPtr, relaxed loads, compare and swap on store
    head: AtomicPtr<Node<T>>,
}

impl<T> UnsyncLinked<T> {
    pub const fn new() -> UnsyncLinked<T> {
        UnsyncLinked { head: AtomicPtr::new(ptr::null_mut()) }
    }

    fn tail(&self) -> (*mut Node<T>, usize) {
        let mut cur_node = self.head.load(Ordering::Relaxed);

        let mut len = 1;

        while let Some(new_node) = Node::get_next_opt(cur_node) {
            cur_node = new_node.load(Ordering::Relaxed);
            len += 1;
        }

        (cur_node, len)
    }

    pub fn len(&self) -> usize {
        self.tail().1
    }

    /// Returns the new length of the list
    pub fn push(&self, val: T) -> usize {
        let new = Node::new(val);
        match self.head.compare_exchange(ptr::null_mut(), new, Ordering::AcqRel, Ordering::Acquire) {
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

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { cur: &self.head, phantom: PhantomData }
    }
}

impl<T> Index<usize> for UnsyncLinked<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let mut cur_node = self.head.load(Ordering::Relaxed);
        if cur_node.is_null() {
            panic!("Index out of bounds");
        }
        for _ in 0..index {
            cur_node = Node::get_next_opt(cur_node).expect("Index out of bounds")
                .load(Ordering::Relaxed);
        }
        unsafe { Node::get_val(cur_node) }
    }
}

impl<T> Drop for UnsyncLinked<T> {
    fn drop(&mut self) {
        let mut cur = self.head.load(Ordering::Relaxed);
        while !cur.is_null() {
            cur = unsafe { Box::from_raw(cur) }.next.load(Ordering::Relaxed);
        }
    }
}

unsafe impl<T> Send for UnsyncLinked<T> {}
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
        let ptr = self.cur.load(Ordering::Relaxed);
        if ptr.is_null() {
            None
        } else {
            let out = unsafe { Node::get_val(ptr) };
            self.cur = Node::get_next(ptr);
            Some(out)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_thread() {
        let list = UnsyncLinked::<u16>::new();
        std::thread::scope(|scope| {
            let list = &list;
            let mut joins = Vec::new();
            for i in 0..100 {
                joins.push(scope.spawn(move || list.push(i)));
            }
            for join in joins {
                join.join().unwrap();
            }
        });
        assert_eq!(list.len(), 100);
    }
}
