use std::ptr;

pub struct Queue<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    len: usize,
}

struct Node<T> {
    elem: T,
    next: *mut Node<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { head: ptr::null_mut(), tail: ptr::null_mut(), len: 0 }
    }

    pub fn push(&mut self, elem: T) {
        // From here on, the node is owned by the queue and reached only through raw pointers.
        let new_tail = Box::into_raw(Box::new(Node { elem, next: ptr::null_mut() }));
        if self.tail.is_null() {
            self.head = new_tail;
        } else {
            // SAFETY: a non-null `tail` points to the last node, which is alive.
            unsafe { (*self.tail).next = new_tail };
        }
        self.tail = new_tail;
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_null() {
            return None;
        }
        // SAFETY: `head` came from `Box::into_raw` in `push`, and it is turned back into a Box
        // only here, once, right before it is unlinked.
        let head = unsafe { Box::from_raw(self.head) };
        self.head = head.next;
        if self.head.is_null() {
            // The last node is gone: `tail` must not keep pointing to freed memory.
            self.tail = ptr::null_mut();
        }
        self.len -= 1;
        Some(head.elem)
    }

    pub fn peek(&self) -> Option<&T> {
        // SAFETY: a non-null `head` points to a live node; the reference borrows `self`.
        unsafe { self.head.as_ref() }.map(|node| &node.elem)
    }

    pub fn peek_back(&self) -> Option<&T> {
        // SAFETY: a non-null `tail` points to a live node; the reference borrows `self`.
        unsafe { self.tail.as_ref() }.map(|node| &node.elem)
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        // Raw pointers own nothing as far as the compiler knows, so free the nodes by hand.
        while self.pop().is_some() {}
    }
}
