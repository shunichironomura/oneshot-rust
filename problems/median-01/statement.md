Compute the median and the mode of a list of integers.

```rust
pub fn median(v: &[i32]) -> Option<f64>
pub fn mode(v: &[i32]) -> Option<i32>
```

- `median` returns the middle value of `v` sorted in ascending order. If `v` has an even number of
  elements, it returns the mean of the two middle values.
- `mode` returns the value that appears most often in `v`. If several values appear equally often,
  it returns the smallest of them.
- Both return `None` if `v` is empty.
- The values can be anywhere in the `i32` range. `median` must return the exact result (it can
  always be represented as an `f64`).

### Examples

- `median(&[3, 1, 2])` is `Some(2.0)`.
- `median(&[4, 1, 3, 2])` is `Some(2.5)`, the mean of `2` and `3`.
- `mode(&[1, 3, 3, 1, 2])` is `Some(1)`: `1` and `3` both appear twice, and `1` is smaller.
