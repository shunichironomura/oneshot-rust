`Lazy` holds a value that is computed only when it is first needed. It is created from a function
`init` that computes the value. The struct is already in the editor with a placeholder field:
replace that field with the fields you need, and implement the four methods.

```rust
pub struct Lazy<T, F: FnOnce() -> T> { /* fields of your choice */ }

impl<T, F: FnOnce() -> T> Lazy<T, F> {
    pub fn new(init: F) -> Self;
    pub fn get(&mut self) -> &T;
    pub fn is_ready(&self) -> bool;
    pub fn into_value(self) -> T;
}
```

- `new` stores `init` without calling it.
- `get` returns a reference to the value. If the value has not been computed yet, `get` first calls
  `init` and keeps the result. Later calls return the kept value without calling `init` again.
- `is_ready` returns `true` if the value has already been computed (that is, `init` has been
  called), and `false` otherwise. It never computes the value itself.
- `into_value` consumes the `Lazy` and returns the value. If the value has not been computed yet, it
  calls `init` now. Otherwise it returns the kept value (the one `get` returned) without calling
  `init` again.
- `init` is called at most once per `Lazy`, by the first `get` or `into_value`. If neither is called,
  for example when a `Lazy` is dropped unused, `init` is never called.
- `init` can be any value whose type implements `FnOnce() -> T`, for example a function, a closure
  or a boxed closure. This includes closures that can be called only once, such as `move || name`,
  which gives away a captured `String`. `T` can be any type: it does not have to implement `Clone`,
  `Copy` or `Default`.

Do not change the signatures, and do not add trait bounds.

### Examples

```rust
let name = String::from("Ferris");
let mut greeting = Lazy::new(move || name + "!");
greeting.is_ready()   // false
greeting.get()        // "Ferris!" (init is called here)
greeting.is_ready()   // true
greeting.get()        // "Ferris!" (init is not called again)
greeting.into_value() // String::from("Ferris!")
```

```rust
let calls = Cell::new(0);
let lazy = Lazy::new(|| {
    calls.set(calls.get() + 1);
    vec![1, 2, 3]
});
// calls.get() is 0
lazy.into_value() // vec![1, 2, 3], and now calls.get() is 1
```
