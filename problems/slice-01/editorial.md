`chunk_by` (stable since Rust 1.77) calls the closure on each pair of neighbors `a` and `b`, and
starts a new group wherever it returns `false`. With `a < b` the groups are exactly the strictly
increasing runs. Every group is a non-empty subslice of `v`, and an empty `v` has no groups at all.

`rising_runs` returns the groups from `chunk_by` as they are, without copying. By lifetime elision,
`fn rising_runs(v: &[i32]) -> Vec<&[i32]>` means
`fn rising_runs<'a>(v: &'a [i32]) -> Vec<&'a [i32]>`: the slices in the result borrow from `v`,
which is exactly what `v.chunk_by(...)` yields.

The traps are in hand-written index loops:

- `for i in 0..v.len() - 1` looks harmless, but for an empty `v`, `v.len() - 1` is `0 - 1` on a
  `usize`. The judge builds with overflow checks, as debug builds do, so this panics with
  `attempt to subtract with overflow`.
- A loop that pushes the last run after the loop returns `[[]]` instead of `[]` for an empty `v`.
- Testing `b - a > 0` instead of `b > a` overflows when `i32::MIN` and `i32::MAX` are neighbors.
  Compare the values directly.

### Alternatives

An index loop works too, as long as the empty slice is handled.

```rust
pub fn rising_runs(v: &[i32]) -> Vec<&[i32]> {
    let mut result = Vec::new();
    if v.is_empty() {
        return result;
    }
    let mut start = 0;
    for i in 1..v.len() {
        if v[i] <= v[i - 1] {
            result.push(&v[start..i]);
            start = i;
        }
    }
    result.push(&v[start..]);
    result
}
```
