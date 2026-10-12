Compute the mode of a list of integers.

```rust
pub fn mode(v: &[i32]) -> Option<i32>
```

- `mode` returns the value that appears most often in `v`. If several values appear equally often,
  it returns the smallest of them.
- It returns `None` if `v` is empty.
- The values can be anywhere in the `i32` range.

### Examples

- `mode(&[1, 3, 3, 1, 2])` is `Some(1)`: `1` and `3` both appear twice, and `1` is smaller.
- `mode(&[1, 2, 3, 3, 2, 3])` is `Some(3)`.
