This code compiles, and when you run it natively it passes every test. But it contains
**undefined behavior (UB)**, and Miri detects it.

`split_in_two` splits a mutable slice at index `mid` and returns two mutable slices: `v[..mid]` and
`v[mid..]`. The caller may read and write through both of them at the same time. You may assume
`mid <= v.len()`.

```rust
pub fn split_in_two(v: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32])
```

Remove the UB so that it works correctly. (You may keep using `unsafe` if you like.)

### Example

With `v = [1, 2, 3, 4, 5]` and `mid = 2`, the halves are `[1, 2]` and `[3, 4, 5]`. Setting
`left[0] = 10` and `right[0] = 30` turns `v` into `[10, 2, 30, 4, 5]`.
