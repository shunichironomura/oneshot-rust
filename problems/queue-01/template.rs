use std::ptr;

pub struct Queue<T> {
    head: Option<Box<Node<T>>>,
    tail: *mut Node<T>,
    len: usize,
}

struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { head: None, tail: ptr::null_mut(), len: 0 }
    }

    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node { elem, next: None });
        // The node itself stays where it is on the heap when the Box is moved below,
        // so this pointer remains valid.
        let raw_tail: *mut Node<T> = &mut *new_tail;
        if self.tail.is_null() {
            self.head = Some(new_tail);
        } else {
            // SAFETY: `tail` points to the last node of the list, which is alive.
            unsafe { (*self.tail).next = Some(new_tail) };
        }
        self.tail = raw_tail;
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;
            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }
            self.len -= 1;
            head.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_back(&self) -> Option<&T> {
        // SAFETY: a non-null `tail` points to the last node, which is alive.
        unsafe { self.tail.as_ref() }.map(|node| &node.elem)
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
