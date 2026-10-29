`windows(3)` yields every window of three neighbors `[v[i], v[i + 1], v[i + 2]]`, without any
index arithmetic on your side. Window `i` is centered on index `i + 1`, hence the `+ 1`. A slice
shorter than three has no windows, so the short inputs need no special case.

The traps are in hand-written index loops:

- `for i in 1..v.len() - 1` looks harmless, but for an empty `v`, `v.len() - 1` is `0 - 1` on a
  `usize`. The judge builds with overflow checks, as debug builds do, so this panics with
  `attempt to subtract with overflow`.
- Testing `v[i] - v[i - 1] > 0` instead of `v[i - 1] < v[i]` overflows when `i32::MIN` and
  `i32::MAX` are neighbors. Compare the values directly.

### Alternatives

An index loop works too, as long as the end of the range cannot underflow. `saturating_sub` stops at
`0` instead.

```rust
pub fn local_peaks(v: &[i32]) -> Vec<usize> {
    (1..v.len().saturating_sub(1))
        .filter(|&i| v[i - 1] < v[i] && v[i] > v[i + 1])
        .collect()
}
```
