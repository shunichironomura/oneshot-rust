The comment in the template says that the node stays where it is on the heap when the `Box` is
moved, so `raw_tail` remains valid. The premise is true, the conclusion is not. A pointer is only
usable while it still has *permission* to access the memory, and Miri's aliasing model (Stacked
Borrows) treats a `Box` like a `&mut`: moving it, or passing it to a function, asserts that the `Box`
is the only way to reach the node, and that revokes every pointer derived from it earlier. Here
`self.head = Some(new_tail)` (or `(*self.tail).next = Some(new_tail)`) moves the `Box` right after
`raw_tail` was taken, so the next `push` writes through a pointer that is no longer valid.

The fix is to stop owning the nodes through `Box` at all. `Box::into_raw` turns the new node into a
raw pointer, and from then on `head`, every `next` and `tail` are plain raw pointers: no `Box` is
left that could be moved and invalidate `tail`. `pop` turns the first node back into a `Box` with
`Box::from_raw`, exactly once, to move the element out and free the node.

Raw pointers own nothing as far as the compiler knows, which has two consequences:

- The nodes are no longer freed automatically. Without `impl Drop`, dropping a non-empty queue leaks
  its nodes and elements, and Miri reports the leak. Popping until the queue is empty frees every
  node and drops every element exactly once.
- When `pop` frees the last node, `tail` must be reset to null. Otherwise the next `push` writes into
  freed memory. `peek_back` uses `as_ref`, which returns `None` for a null pointer.

Half fixes that keep the `Box` links fail as well. Taking the pointer only after the move
(`self.tail = &mut **slot`) works until that `Box` is moved again: when `pop` moves the first node
out of its `Box` (`let head = *head;`), the `Box` of the second node (its `next` field) moves along
with it, which revokes `tail` when that node is the last one. Even moving a queue with a single
element (returning it from a function, pushing it into another queue) moves the `head` field, the
`Box` that owns the last node. Taking the pointer first and then calling `Box::into_raw(boxed)` or
`mem::forget(boxed)` fails for the same reason: passing the `Box` to a function moves it. Use the
pointer that `Box::into_raw` returns.

### Alternatives

`Option<NonNull<Node<T>>>` says "no node" with `None` instead of a null pointer, and `Box::leak`
hands over the node just like `Box::into_raw`. Unlike `*mut T`, `NonNull<T>` is covariant, so this
`Queue<&'static str>` can be used where a `Queue<&'a str>` is expected, as with `Box`.

```rust
use std::ptr::NonNull;

pub struct Queue<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

struct Node<T> {
    elem: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { head: None, tail: None, len: 0 }
    }

    pub fn push(&mut self, elem: T) {
        let node = NonNull::from(Box::leak(Box::new(Node { elem, next: None })));
        match self.tail {
            // SAFETY: `tail` points to the last node, which is alive.
            Some(mut tail) => unsafe { tail.as_mut().next = Some(node) },
            None => self.head = Some(node),
        }
        self.tail = Some(node);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = self.head?;
        // SAFETY: the node was leaked in `push` and is reclaimed only here, once.
        let node = unsafe { Box::from_raw(head.as_ptr()) };
        self.head = node.next;
        if self.head.is_none() {
            self.tail = None;
        }
        self.len -= 1;
        Some(node.elem)
    }

    pub fn peek(&self) -> Option<&T> {
        // SAFETY: the node is alive while `self` is borrowed.
        self.head.map(|node| unsafe { &node.as_ref().elem })
    }

    pub fn peek_back(&self) -> Option<&T> {
        // SAFETY: the node is alive while `self` is borrowed.
        self.tail.map(|node| unsafe { &node.as_ref().elem })
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}
```

Without any raw pointers, the last node can be shared between the node before it and `tail` with
`Rc`. The link to the next node is changed through a shared `Rc`, so it needs a `RefCell`, but
`elem` stays outside the cell, so `peek` can still return a plain `&T`. Once `pop` has removed the
first node from `head` (and from `tail`, if it was the last one), it holds the only `Rc` to it, and
`Rc::into_inner` gives the node back. There are no cycles, so no `Drop` impl is needed. The price is
reference counting and borrow checks at run time.

```rust
use std::cell::RefCell;
use std::rc::Rc;

pub struct Queue<T> {
    head: Option<Rc<Node<T>>>,
    tail: Option<Rc<Node<T>>>,
    len: usize,
}

struct Node<T> {
    elem: T,
    next: RefCell<Option<Rc<Node<T>>>>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { head: None, tail: None, len: 0 }
    }

    pub fn push(&mut self, elem: T) {
        let node = Rc::new(Node { elem, next: RefCell::new(None) });
        match self.tail.take() {
            Some(old_tail) => *old_tail.next.borrow_mut() = Some(Rc::clone(&node)),
            None => self.head = Some(Rc::clone(&node)),
        }
        self.tail = Some(node);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let old_head = self.head.take()?;
        self.head = old_head.next.borrow_mut().take();
        if self.head.is_none() {
            // `tail` also points to the last node; let go of it too.
            self.tail = None;
        }
        self.len -= 1;
        // Nothing else refers to the node any more, so it can be unwrapped.
        Rc::into_inner(old_head).map(|node| node.elem)
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_deref().map(|node| &node.elem)
    }

    pub fn peek_back(&self) -> Option<&T> {
        self.tail.as_deref().map(|node| &node.elem)
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
```

Outside of this exercise, `std::collections::VecDeque` (a growable ring buffer) is the usual choice
for a queue.
