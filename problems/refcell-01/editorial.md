`LimitTracker<'a, T>` stores a shared reference `&'a T`, so a tracker can never outlive its
messenger, and any number of trackers (and the test itself) can use the same recorder at once. The
price is that `send` only gets `&self`. A `Recorder` with a plain `Vec<String>` field therefore
cannot record anything: `self.sent.push(...)` in `send` is rejected with "cannot borrow `self.sent`
as mutable, as it is behind a `&` reference". The compiler suggests changing the trait to
`&mut self`, but that is not allowed here and would not work anyway: while trackers hold
`&recorder`, nobody can get a `&mut` borrow of it.

This is what `RefCell` is for (the Book's mock-object pattern). A `RefCell<Vec<String>>` can be
changed through a shared reference: `borrow_mut()` returns a guard (`RefMut`) that acts as a
`&mut Vec<String>`, and `borrow()` returns a guard (`Ref`) for reading. `messages` must return an
owned `Vec<String>`, so it clones the vector; the `Ref` guard itself cannot be returned.

`RefCell` does not switch the borrowing rules off; it checks them at run time. If a guard is still
alive, a conflicting `borrow_mut()` panics with "RefCell already borrowed". In the solution each
guard is a temporary that is dropped at the end of its statement, so they never overlap. Code such
as `let sent = self.sent.borrow();` followed by `self.sent.borrow_mut().push(...)` while `sent` is
still in use compiles, but panics on the first message.

### Alternatives

`Cell` also works for values that are not `Copy`, but you cannot borrow its contents. Move the
vector out with `take` (which leaves an empty one behind), change it, and put it back with `set`.
There are no guards, so nothing can panic, at the cost of moving the vector in and out.

```rust
use std::cell::Cell;

pub struct Recorder {
    sent: Cell<Vec<String>>,
}

impl Recorder {
    pub fn new() -> Recorder {
        Recorder { sent: Cell::new(Vec::new()) }
    }

    pub fn messages(&self) -> Vec<String> {
        let sent = self.sent.take();
        let copy = sent.clone();
        self.sent.set(sent);
        copy
    }
}

impl Messenger for Recorder {
    fn send(&self, msg: &str) {
        let mut sent = self.sent.take();
        sent.push(msg.to_string());
        self.sent.set(sent);
    }
}
```

A `Mutex<Vec<String>>` also allows changes through `&self`, and it is the tool to use when the
messenger is shared between threads. In single-threaded code, `RefCell` is the lighter choice.

```rust
use std::sync::Mutex;

pub struct Recorder {
    sent: Mutex<Vec<String>>,
}

impl Recorder {
    pub fn new() -> Recorder {
        Recorder { sent: Mutex::new(Vec::new()) }
    }

    pub fn messages(&self) -> Vec<String> {
        self.sent.lock().unwrap().clone()
    }
}

impl Messenger for Recorder {
    fn send(&self, msg: &str) {
        self.sent.lock().unwrap().push(msg.to_string());
    }
}
```

With `RefCell`, `messages` can also be written as `self.sent.borrow().to_vec()` or
`self.sent.clone().into_inner()`.
