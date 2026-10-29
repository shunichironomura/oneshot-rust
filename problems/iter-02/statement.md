The code below does not compile. Fix it.

Three small functions answer questions about slices.

```rust
pub fn first_negative(v: &[i32]) -> Option<i32>
pub fn position_of_max(v: &[i32]) -> Option<usize>
pub fn count_long(words: &[&str], min: usize) -> usize
```

- `first_negative` returns the first element that is less than `0`, or `None` if there is none.
- `position_of_max` returns the index of the largest element. If the largest value appears more than
  once, it returns the index of its first occurrence. For an empty slice it returns `None`.
- `count_long` returns how many words are at least `min` bytes long, as returned by `len()`: `"é"`
  is 2 bytes long.

Do not change the signatures.

### Examples

- `first_negative(&[3, -1, -5])` returns `Some(-1)`.
- `position_of_max(&[1, 9, 9])` returns `Some(1)`.
- `count_long(&["a", "abc", "de"], 2)` returns `2`.
