Implement the function `rising_runs`, which splits a slice into its rising runs.

```rust
pub fn rising_runs(v: &[i32]) -> Vec<&[i32]>
```

- `rising_runs` splits `v` into strictly increasing runs, each as long as possible: a new run starts
  at every element that is less than or equal to the element before it. It returns the runs from
  left to right as subslices of `v`.
- `v` can have any length, including 0 and 1, and its values can be any `i32`, including
  `i32::MIN` and `i32::MAX`. It must not panic. For an empty `v`, it returns an empty vector.

### Examples

`rising_runs(&[1, 2, 2, 3, 1])` is `[[1, 2], [2, 3], [1]]`: the second `2` is not greater than the
first one, so it starts a new run.
