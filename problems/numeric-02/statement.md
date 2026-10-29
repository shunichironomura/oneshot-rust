Implement the function `sum_checked`. It is a short formula, but it has to be right for **every**
allowed input, including sums far beyond the range of `u8`.

```rust
pub fn sum_checked(xs: &[u8]) -> Option<u8>
```

- `sum_checked(xs)` returns the sum of `xs` in `Some` if it is at most 255, and `None` otherwise.
  The sum is meant mathematically, as if computed with integers of unlimited size.
- The sum of an empty slice is `0`, so the result is `Some(0)`.
- It must not panic for any input, however long `xs` is.

### Examples

`sum_checked(&[100, 155])` is `Some(255)`, and `sum_checked(&[200, 100])` is `None`.
