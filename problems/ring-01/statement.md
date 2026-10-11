A ring buffer keeps the most recent elements in an array of fixed size. Implement `Ring<T, N>`, a
first-in, first-out queue that holds at most `N` elements, where the capacity `N` is a const generic
parameter. The struct, the constant `CAPACITY` and the method signatures are already in the editor.

```rust
pub struct Ring<T, const N: usize> {
    buf: [Option<T>; N],
    start: usize,
    len: usize,
}

impl<T, const N: usize> Ring<T, N> {
    pub const CAPACITY: usize = N;

    pub fn new() -> Self;
    pub fn push(&mut self, x: T) -> Option<T>;
    pub fn pop(&mut self) -> Option<T>;
    pub fn len(&self) -> usize;
    pub fn is_full(&self) -> bool;
    pub fn iter(&self) -> impl Iterator<Item = &T> + '_;
}
```

- `new` returns an empty ring.
- `push(x)` adds `x` as the newest element. If the ring was not full, it returns `None`. If it was
  full (it held `N` elements), it also removes the oldest element and returns it as `Some`, so the
  ring still holds `N` elements.
- `pop` removes the oldest element and returns it. On an empty ring it returns `None`, and the ring
  stays empty and usable.
- `len` returns the number of elements, and `is_full` tells whether that number is `N`.
- `iter` returns an iterator over references to the elements, from the oldest to the newest. It
  does not change the ring.
- `Ring::<T, N>::CAPACITY` is `N` (this is already written).
- `T` can be any type, including types that implement none of `Copy`, `Clone`, `Default` and
  `Debug`; the harness uses such types. Do not add bounds on `T`.
- Keep the fields of `Ring` as they are: the elements are stored in the array `buf` inside the
  struct itself, not in a collection on the heap such as `Vec` or `VecDeque`. `start` and `len` are
  meant for the index of the oldest element in `buf` and for the number of elements.
- `N` is at least 1.

### Example

```rust
let mut r: Ring<i32, 2> = Ring::new();
r.push(1); // None
r.push(2); // None
r.push(3); // Some(1): the ring was full, so the oldest element 1 is evicted
// Now r.len() is 2, r.is_full() is true, and r.iter() yields &2, then &3.
r.pop();   // Some(2)
// Ring::<i32, 3>::CAPACITY is 3.
```
