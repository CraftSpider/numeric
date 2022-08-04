use std::marker::PhantomData;
use std::ops::Index;
use std::ptr::NonNull;

type NodePtr<T> = Option<NonNull<Node<T>>>;

pub struct Node<T> {
    next: NodePtr<T>,
    val: T,
}

impl<T> Node<T> {
    fn new(val: T) -> NonNull<Node<T>> {
        let raw_ptr = Box::into_raw(Box::new(Node {
            next: None,
            val,
        }));
        // SAFETY: Box::into_raw will produce a non-null pointer
        unsafe { NonNull::new_unchecked(raw_ptr) }
    }

    /// Set the next node, without creating a mutable reference
    fn set_next(this: NonNull<Node<T>>, new: NodePtr<T>) {
        unsafe { (*this.as_ptr()).next = new };
    }

    /// Get the next node, without creating a reference
    fn get_next(this: NonNull<Node<T>>) -> NodePtr<T> {
        unsafe { (*this.as_ptr()).next }
    }

    /// Create a reference to the value in this node
    ///
    /// # Safety
    ///
    /// Pointer must be valid, and no mutable references must exist
    unsafe fn get_val<'a>(this: NonNull<Node<T>>) -> &'a T {
        &(*this.as_ptr()).val
    }
}

pub struct UnsyncLinked<T> {
    // TODO: AtomicPtr, relaxed loads, compare and swap on store
    head: NonNull<Node<T>>,
}

impl<T> UnsyncLinked<T> {
    pub fn new_with(first: T) -> UnsyncLinked<T> {
        UnsyncLinked {
            head: Node::new(first),
        }
    }

    fn tail(&self) -> (NonNull<Node<T>>, usize) {
        let mut cur_node = self.head;
        let mut len = 1;
        while let Some(new_node) = Node::get_next(cur_node) {
            cur_node = new_node;
            len += 1;
        }
        (cur_node, len)
    }

    pub fn len(&self) -> usize {
        self.tail().1
    }

    /// Returns the new length of the list
    pub fn push(&self, val: T) -> usize {
        let (tail, len) = self.tail();
        Node::set_next(tail, Some(Node::new(val)));
        len + 1
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { cur: Some(self.head), phantom: PhantomData }
    }
}

impl<T> Index<usize> for UnsyncLinked<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let mut cur_node = self.head;
        for _ in 0..index {
            cur_node = Node::get_next(cur_node).expect("Index out of bounds");
        }
        unsafe { Node::get_val(cur_node) }
    }
}

unsafe impl<T> Send for UnsyncLinked<T> {}
unsafe impl<T> Sync for UnsyncLinked<T> {}

pub struct Iter<'a, T> {
    cur: NodePtr<T>,
    phantom: PhantomData<&'a ()>,
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: 'a,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cur {
            Some(cur) => {
                let out = unsafe { Node::get_val(cur) };
                self.cur = Node::get_next(cur);
                Some(out)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_thread() {
        let list = UnsyncLinked::<u8>::new_with(0);
        std::thread::scope(|scope| {
            let list = &list;
            let mut joins = Vec::new();
            for i in 0..99 {
                joins.push(scope.spawn(move || list.push(i)));
            }
            for join in joins {
                join.join().unwrap();
            }
            assert_eq!(list.len(), 100);
        });
    }
}
