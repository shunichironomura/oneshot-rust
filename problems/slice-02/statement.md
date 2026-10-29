A *peak* of a slice is an element that is greater than both of its neighbors. Implement the
function `local_peaks`, which finds the peaks.

```rust
pub fn local_peaks(v: &[i32]) -> Vec<usize>
```

- `local_peaks` returns the positions of the peaks in ascending order: every index `i` such that
  `v[i - 1] < v[i]` and `v[i] > v[i + 1]`. The first and last elements are never peaks, since they
  lack a neighbor on one side. A flat top is not a peak either: `[1, 4, 4, 1]` has no peaks.
- `v` can have any length, including 0, 1 and 2, and its values can be any `i32`, including
  `i32::MIN` and `i32::MAX`. It must not panic. For an empty `v`, it returns an empty vector.

### Examples

`local_peaks(&[1, 3, 2, 5, 1])` is `[1, 3]`, the positions of `3` and `5`.
