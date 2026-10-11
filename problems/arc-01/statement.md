Build `MyArc<T>`, a reference-counted pointer that can be shared between threads like
`std::sync::Arc<T>` (without weak references), from scratch: on top of a heap allocation, raw
pointers and atomics.

```rust
pub struct MyArc<T> { /* private fields of your choice */ }

impl<T> MyArc<T> {
    pub fn new(value: T) -> Self;
    pub fn strong_count(this: &Self) -> usize;
    pub fn get_mut(this: &mut Self) -> Option<&mut T>;
    pub fn ptr_eq(a: &Self, b: &Self) -> bool;
}

impl<T> Clone for MyArc<T> { /* to implement */ }
impl<T> Deref for MyArc<T> { type Target = T; /* to implement */ }
impl<T> Drop for MyArc<T> { /* to implement */ }
```

- `MyArc::new(value)` moves `value` into a new heap allocation and returns the first handle to it.
- `clone` returns another handle to the same value. It does not clone the value, and `T` does not
  need to implement `Clone`.
- A handle dereferences to its value: `*a`, `a.len()` and so on.
- `MyArc::strong_count(&a)` is the number of handles to the value of `a` that currently exist,
  including `a` itself.
- `MyArc::get_mut(&mut a)` returns a mutable reference to the value if `a` is its only handle, and
  `None` otherwise.
- `MyArc::ptr_eq(&a, &b)` is `true` if `a` and `b` are handles to the same value. A value is shared
  by the handle that `new` returned and every handle cloned from it, directly or through other
  clones. Values created by separate calls to `new` are never the same, even when they are equal or
  zero-sized.
- Dropping a handle that is not the last one only updates the count. When the last handle is
  dropped, the value is dropped exactly once and the allocation is freed, on whichever thread that
  happens.
- Handles may be cloned, used, sent to other threads and dropped on several threads at the same
  time. `MyArc<T>` is `Send` and `Sync` when `T` is both `Send` and `Sync`, and neither of them
  otherwise (like `Arc<T>`).
- `MyArc<T>` is covariant in `T`, like `Arc<T>`: a `MyArc<&'static str>` can be used where a
  `MyArc<&'a str>` is expected. For example, this function must compile:

```rust
fn shorten<'a>(a: MyArc<&'static str>) -> MyArc<&'a str> {
    a
}
```

Do not use `Arc`, `Rc` or any other reference-counted type from `std`; you will need `unsafe`. The
private struct `Inner<T>` in the editor is only a suggestion: you may change or remove it.
Undefined behavior, data races and memory leaks are detected by Miri and fail the submission. The
count never gets anywhere near overflowing.

### Examples

```rust
let a = MyArc::new(String::from("hello"));
let b = MyArc::clone(&a);
// *b == "hello", a.len() == 5, MyArc::strong_count(&a) == 2, MyArc::ptr_eq(&a, &b) is true
drop(a);
// MyArc::strong_count(&b) == 1
let c = MyArc::new(String::from("hello"));
// MyArc::ptr_eq(&b, &c) is false

let mut v = MyArc::new(vec![1, 2]);
MyArc::get_mut(&mut v).unwrap().push(3);
let w = MyArc::clone(&v);
// MyArc::get_mut(&mut v) is None
drop(w);
MyArc::get_mut(&mut v).unwrap().push(4);
// *v == [1, 2, 3, 4]

let words = MyArc::new(vec!["a", "bb", "ccc"]);
let (n, joined) = thread::scope(|s| {
    let mine = MyArc::clone(&words);
    let t1 = s.spawn(move || mine.len()); // a handle moved to another thread
    let t2 = s.spawn(|| words.concat()); // a &MyArc shared with another thread
    (t1.join().unwrap(), t2.join().unwrap())
});
// n == 3, joined == "abbccc", MyArc::strong_count(&words) == 1
```
