Calling an `FnOnce` consumes it, so the closure has to be moved out of the struct to be called. A
field cannot be moved out through `&mut self`, though: that would leave a hole in a struct that
someone else still owns. Storing `init: F` and writing `(self.init)()` in `get` is rejected with
E0507 ("cannot move out of `self.init` which is behind a mutable reference").

The fix is to store `Option<F>`. `Option::take` moves the closure out and leaves `None` behind, so
the struct stays whole, and the `None` also records that `init` has been used. The value is kept in
an `Option<T>`, and `get_or_insert_with` does the rest: it calls its closure only when the value is
`None`, stores the result and returns a reference to it. That closure uses only the field
`self.init`, and since closures capture individual fields, it does not conflict with
`self.value`, which `get_or_insert_with` is borrowing mutably at the same time.

Beware of this shorter version, which compiles:

```rust
self.value.get_or_insert_with(self.init.take().unwrap())
```

`F` is itself an `FnOnce() -> T`, so it fits the parameter, but arguments are evaluated **before**
the call. Every `get` takes `init` out, and the second call panics on `unwrap` of `None`. The same
goes for `self.value.unwrap_or_else(self.init.unwrap())` in `into_value`: it panics once `get` has
been called. Wrapping the expression in a closure (`|| (self.init.take().unwrap())()`) delays it
until it is really needed.

`into_value` takes `self` by value, so it may move the fields out directly. (This works because
`Lazy` does not implement `Drop`: moving a field out of a type that does is rejected with E0509.)
The `expect` in `get` and `into_value` can only fail if an earlier call to `init` panicked and left
neither a closure nor a value.

Adding a bound such as `F: Fn` or `F: Clone` would let you call `init` without moving it, and
`T: Clone` would let `into_value` clone the value, but the tests use closures that give away what
they captured, and values that cannot be cloned.

The standard library has this type too: `std::cell::LazyCell` (and its thread-safe sibling
`std::sync::LazyLock`), which computes its value through `&self` by using interior mutability.

### Alternatives

Take the closure out first and check whether there was one. `into_value` can reuse `get`
(`mut self` only makes the binding mutable; the signature stays the same).

```rust
pub fn get(&mut self) -> &T {
    if let Some(init) = self.init.take() {
        self.value = Some(init());
    }
    self.value.as_ref().expect("an earlier call to init panicked")
}

pub fn into_value(mut self) -> T {
    self.get();
    self.value.expect("an earlier call to init panicked")
}
```

An enum makes the states explicit, so a closure and a value can never be present at the same time.
`mem::replace` moves the closure out through `&mut self`, but it needs something to leave behind,
which is what the `Running` state is for.

```rust
use std::mem;

enum State<T, F> {
    Pending(F),
    Running,
    Ready(T),
}

pub struct Lazy<T, F: FnOnce() -> T> {
    state: State<T, F>,
}

impl<T, F: FnOnce() -> T> Lazy<T, F> {
    pub fn new(init: F) -> Self {
        Lazy { state: State::Pending(init) }
    }

    pub fn get(&mut self) -> &T {
        if let State::Pending(_) = self.state {
            // Take the closure out, leaving `Running` behind while `init` runs.
            if let State::Pending(init) = mem::replace(&mut self.state, State::Running) {
                self.state = State::Ready(init());
            }
        }
        match &self.state {
            State::Ready(value) => value,
            _ => panic!("an earlier call to init panicked"),
        }
    }

    pub fn is_ready(&self) -> bool {
        matches!(self.state, State::Ready(_))
    }

    pub fn into_value(self) -> T {
        match self.state {
            State::Pending(init) => init(),
            State::Ready(value) => value,
            State::Running => panic!("an earlier call to init panicked"),
        }
    }
}
```
