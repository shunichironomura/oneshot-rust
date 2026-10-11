`#[derive(Clone)]` does not check which bounds the fields really need. On a generic struct it adds
a bound to every type parameter and generates `impl<T: Clone> Clone for Handle<T>`, although the
only field, `Rc<RefCell<T>>`, can be cloned for any `T`: cloning an `Rc` copies the pointer and
increments the count. Inside `impl<T> Handle<T>` nothing is known about `T`, so there `Handle<T>`
is not `Clone`. Every built-in derive works this way (it is known as "imperfect derive"): on a
struct with a `PhantomData<U>` field, `#[derive(PartialEq)]` requires `U: PartialEq` too.

The surprise is that `self.clone()` still finds a `clone` method. Method lookup tries the receiver
with an extra `&` too, and `&Handle<T>` is `Clone` for any `T` (every shared reference is `Copy`).
So `self.clone()` in `share`, where `self` is a `&Handle<T>`, copies the **reference**, and the
compiler complains about the result instead: "expected `Handle<T>`, found `&Handle<T>`". In
`fan_out`, `h.clone()` yields `&Handle<T>` as well, so `collect` cannot build a `Vec<Handle<T>>`.

The fix is to write the `Clone` impl by hand, without the bound:

```rust
impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Handle { inner: Rc::clone(&self.inner) }
    }
}
```

Now `self.clone()` and `h.clone()` return a `Handle<T>`, and `share` and `fan_out` work unchanged.
`Rc::clone(&self.inner)` does the same as `self.inner.clone()`, but makes it clear that only the
pointer is copied. Keep `#[derive(Default)]`: its bound `T: Default` is exactly right, because a new
handle needs a `T::default()`.

Fixes that only make the editor code compile are not enough:

- Adding `T: Clone` to `impl<T> Handle<T>` and `fan_out` compiles, but `Handle<Counter>` is still
  not `Clone`, so the tests do not compile.
- Writing `Handle { inner: Rc::clone(&self.inner) }` in `share` and calling `h.share()` in `fan_out`
  also compiles, but the tests call `clone()` on a `Handle<Counter>` too.
- `fn clone(&self) -> Self { self.share() }` while `share` still calls `self.clone()` recurses
  forever.

### Alternatives

Put the logic in `share` and let `clone` call it. `Default` can be written by hand too, if you
prefer to see every bound.

```rust
use std::cell::RefCell;
use std::rc::Rc;

pub struct Handle<T> {
    inner: Rc<RefCell<T>>,
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        self.share()
    }
}

impl<T: Default> Default for Handle<T> {
    fn default() -> Self {
        Handle::new(T::default())
    }
}

impl<T> Handle<T> {
    pub fn new(value: T) -> Self {
        Handle { inner: Rc::new(RefCell::new(value)) }
    }

    pub fn share(&self) -> Handle<T> {
        Handle { inner: Rc::clone(&self.inner) }
    }

    pub fn update(&self, f: impl FnOnce(&mut T)) {
        f(&mut self.inner.borrow_mut());
    }

    pub fn read<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        f(&self.inner.borrow())
    }

    pub fn handles(&self) -> usize {
        Rc::strong_count(&self.inner)
    }
}

pub fn fan_out<T>(h: &Handle<T>, n: usize) -> Vec<Handle<T>> {
    (0..n).map(|_| h.share()).collect()
}
```

Once `Handle<T>` is `Clone` for every `T`, `vec!` can also make the copies. It clones its element
`n - 1` times and moves the original into the last slot (or drops it when `n` is `0`), so the count
of handles stays right.

```rust
pub fn fan_out<T>(h: &Handle<T>, n: usize) -> Vec<Handle<T>> {
    vec![h.clone(); n]
}
```
