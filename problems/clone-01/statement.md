The code below does not compile. Fix it.

`Handle<T>` is a handle to a value of type `T` that several owners share. The value lives in an
`Rc<RefCell<T>>`, and every handle made from another one points at the same value.

```rust
pub struct Handle<T> { /* private */ }

impl<T> Handle<T> {
    pub fn new(value: T) -> Self;
    pub fn share(&self) -> Handle<T>;
    pub fn update(&self, f: impl FnOnce(&mut T));
    pub fn read<R>(&self, f: impl FnOnce(&T) -> R) -> R;
    pub fn handles(&self) -> usize;
}

pub fn fan_out<T>(h: &Handle<T>, n: usize) -> Vec<Handle<T>>
```

- `new(value)` creates the first handle to a new value.
- `h.clone()` and `h.share()` both return a new handle to the same value as `h`. The value itself is
  never copied. `Handle<T>` must implement `Clone` for **every** `T`, including types that do not
  implement `Clone`, such as `Counter` in the example. The tests call `clone()` on such handles
  directly, too.
- `Handle<T>` also implements `Default` when `T` does: `Handle::default()` creates the first handle
  to a new value `T::default()`.
- `update(f)` calls `f` with a mutable reference to the value. `read(f)` calls `f` with a shared
  reference to the value and returns what `f` returns. A change made through one handle is visible
  through every handle to the same value.
- `handles()` returns how many handles to the same value are alive, including `self`. Dropping a
  handle lowers the count, and the value is dropped together with its last handle.
- `fan_out(h, n)` returns `n` new handles to the value of `h` (an empty `Vec` when `n` is `0`).

Do not change the public signatures. Anything else may be changed as long as the rules above hold.

### Example

```rust
struct Counter { n: u32 } // implements neither Clone nor Default

let h = Handle::new(Counter { n: 0 });
let v = fan_out(&h, 2);
v[0].update(|c| c.n += 5);
h.read(|c| c.n)  // 5
h.handles()      // 3

let a = Handle::new(Counter { n: 1 });
let b = a.clone();
b.update(|c| c.n *= 10);
a.read(|c| c.n)  // 10
a.handles()      // 2
drop(b);
a.handles()      // 1
```
