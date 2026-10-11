Build `MyVec<T>`, a growable array like `Vec<T>`, from scratch with `std::alloc` and raw
pointers. The struct with its three fields is already in the editor.

```rust
pub struct MyVec<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self;
    pub fn len(&self) -> usize;
    pub fn capacity(&self) -> usize;
    pub fn push(&mut self, x: T);
    pub fn pop(&mut self) -> Option<T>;
    pub fn get(&self, i: usize) -> Option<&T>;
    pub fn as_slice(&self) -> &[T];
}

impl<T> Drop for MyVec<T> { /* to implement */ }
```

- Do not store the elements in `Vec`, `Box`, `String` or any other std collection. Allocate the
  buffer yourself with `std::alloc` (`alloc`, `realloc`, `dealloc`).
- `new` creates an empty `MyVec` without allocating anything. Its capacity is `0` (except for
  zero-sized types, see below).
- `len` is the number of elements, and `capacity` is how many elements fit in the buffer.
- `push` appends `x` at the end. If the `MyVec` is full (`len() == capacity()`), the capacity grows
  first: `0` becomes `4`, and any other capacity doubles (`4` → `8` → `16` → ...). The capacity
  changes at no other time; in particular, `pop` never shrinks it.
- `pop` removes the last element and returns it. It returns `None` if the `MyVec` is empty.
- `get(i)` returns the element at index `i`, or `None` if `i >= len()`. `as_slice` returns all the
  elements in order (an empty slice if there are none).
- For a zero-sized `T` such as `()`, `capacity()` is always `usize::MAX`, and no memory is ever
  allocated. Everything else works as usual.
- When a `MyVec` is dropped, each element still in it is dropped exactly once (in any order), and
  its buffer is freed.
- Undefined behavior and memory leaks are detected by Miri and fail the submission. Allocation
  failure and capacity overflow are not tested; you may simply panic.

### Examples

```rust
let mut v = MyVec::new();
// len() is 0, capacity() is 0
for s in ["a", "b", "c", "d", "e"] {
    v.push(String::from(s));
}
// capacity() after each push: 4, 4, 4, 4, 8
// len() is 5, as_slice() is ["a", "b", "c", "d", "e"]
v.pop(); // Some("e")
v.pop(); // Some("d")
// get(2) is Some("c"), get(3) is None, len() is 3, capacity() is 8

let mut z: MyVec<()> = MyVec::new();
// capacity() is usize::MAX
z.push(());
z.push(());
// len() is 2, capacity() is still usize::MAX
z.pop(); // Some(())
// len() is 1
```
