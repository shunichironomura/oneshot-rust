Implement a function that pairs up the elements of two arrays. The length is a const generic
parameter, so the function works for arrays of any length.

```rust
pub fn zip_arrays<A, B, const N: usize>(a: [A; N], b: [B; N]) -> [(A, B); N]
```

- `zip_arrays` pairs up the elements at the same position: `result[i]` is `(a[i], b[i])`.
- `A` and `B` can be any types, including types that implement neither `Copy` nor `Clone`.
- It must also work for empty arrays (`N = 0`).

### Examples

`zip_arrays([1, 2], ["a", "b"])` returns `[(1, "a"), (2, "b")]`.
