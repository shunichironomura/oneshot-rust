`SortedVec<T>` is a vector whose elements are always in ascending order. Its contents change only
through its own methods, but it can be read like a slice: it dereferences to `[T]`, so read-only
slice methods such as `len`, `iter`, `first`, `contains` and `windows`, and indexing such as
`sv[0]`, work on it directly, and `&sv` can be passed to a function that takes a `&[T]`. Implement
it (the struct and the `impl` blocks are already in the editor).

```rust
pub struct SortedVec<T> {
    items: Vec<T>,
}

impl<T: Ord> SortedVec<T> {
    pub fn new() -> Self;
    pub fn insert(&mut self, value: T) -> usize;
    pub fn remove(&mut self, value: &T) -> bool;
}

impl<T: Ord> From<Vec<T>> for SortedVec<T> {
    fn from(items: Vec<T>) -> Self;
}

impl<T> SortedVec<T> {
    pub fn into_vec(self) -> Vec<T>;
}

impl<T> Deref for SortedVec<T> {
    type Target = [T];
    fn deref(&self) -> &[T];
}
```

- `new` returns an empty `SortedVec`.
- `insert` puts `value` after every element equal to it and before every greater element, and
  returns the index where `value` now is.
- `remove` removes one element equal to `value`, the leftmost one, and returns `true`. If no
  element is equal to `value`, it changes nothing and returns `false`.
- `SortedVec::from(v)` holds the elements of `v` in ascending order. Equal elements keep the order
  they had in `v`.
- `into_vec` returns the elements as a `Vec<T>`, and `deref` as a slice, both in ascending order.
- `T` can be any type that implements `Ord`. The harness uses `i32`, `String` and a type of its
  own, `Key(u8, char)`, which is ordered by its number only: `Key(2, 'a')` and `Key(2, 'b')` are
  equal, but the letter still tells them apart.
- Keep `items` private and do not implement `DerefMut`: changing elements in place could break the
  order.

### Examples

```rust
let mut sv = SortedVec::from(vec![3, 1, 2]); // &*sv is [1, 2, 3]
sv.insert(2)   // 2; sv is now [1, 2, 2, 3]
sv.remove(&2)  // true; sv is now [1, 2, 3]
sv.remove(&5)  // false
sv.len()       // 3
sv[0]          // 1
```

```rust
let mut sv = SortedVec::from(vec![Key(2, 'a'), Key(1, 'x'), Key(3, 'y')]);
// the letters are x, a, y
sv.insert(Key(2, 'b'))   // 2; the letters are now x, a, b, y
sv.remove(&Key(2, 'z'))  // true; the letters are now x, b, y
```
