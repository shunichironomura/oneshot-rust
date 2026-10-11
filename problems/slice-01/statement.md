Implement four functions that look at neighboring elements of a slice: runs of equal values, rising
runs, peaks, and pairs.

```rust
pub fn runs(v: &[i32]) -> Vec<(i32, usize)>
pub fn rising_runs(v: &[i32]) -> Vec<&[i32]>
pub fn local_peaks(v: &[i32]) -> Vec<usize>
pub fn pairs(v: &[i32]) -> (Vec<(i32, i32)>, Option<i32>)
```

- `runs` splits `v` into groups of consecutive equal values, each as long as possible, and returns
  `(value, length)` for each group, from left to right. Equal values that are not next to each other
  belong to different groups.
- `rising_runs` splits `v` into strictly increasing runs, each as long as possible: a new run starts
  at every element that is less than or equal to the element before it. It returns the runs from
  left to right as subslices of `v`.
- `local_peaks` returns the positions of the peaks in ascending order: every index `i` such that
  `v[i - 1] < v[i]` and `v[i] > v[i + 1]`. The first and last elements are never peaks, since they
  lack a neighbor on one side. A flat top is not a peak either: `[1, 4, 4, 1]` has no peaks.
- `pairs` splits `v` from the start into non-overlapping pairs `(v[0], v[1])`, `(v[2], v[3])`, and
  so on. If the length of `v` is odd, the last element is left over and is returned in `Some`;
  otherwise the second part of the result is `None`.
- `v` can have any length, including 0, 1 and 2, and its values can be any `i32`, including
  `i32::MIN` and `i32::MAX`. No function may panic. For an empty `v`, every vector in the results is
  empty, and `pairs` returns `(vec![], None)`.

### Examples

- `runs(&[1, 1, 2, 3, 3, 3])` is `[(1, 2), (2, 1), (3, 3)]`.
- `rising_runs(&[1, 2, 2, 3, 1])` is `[[1, 2], [2, 3], [1]]`: the second `2` is not greater than the
  first one, so it starts a new run.
- `local_peaks(&[1, 3, 2, 5, 1])` is `[1, 3]`, the positions of `3` and `5`.
- `pairs(&[1, 2, 3, 4, 5])` is `([(1, 2), (3, 4)], Some(5))`.
