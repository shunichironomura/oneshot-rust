Implement the function `k_smallest`.

```rust
pub fn k_smallest(v: &[i32], k: usize) -> Vec<i32>
```

- `k_smallest` returns the `k` smallest values of `v` in ascending order, keeping duplicates.
- `v` may hold any `i32` values (including `i32::MIN` and `i32::MAX`), and `k` may be any `usize`.
- If `k` is `v.len()` or more, it returns all the values of `v`, sorted.

You may use anything from `std`, for example `std::collections::BinaryHeap`.

### Examples

`k_smallest(&[5, 1, 4, 1, 3], 3)` returns `[1, 1, 3]`.
