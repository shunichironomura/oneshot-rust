A ring buffer has `len` slots, and positions are counted around the ring in both directions: past
the last slot you are back at the first, and `-1` is the last slot. Implement the function
`wrap_index`, which turns a position into a slot index. It has to be right for **every** allowed
input, including the largest and smallest `i64` values.

```rust
pub fn wrap_index(i: i64, len: usize) -> usize
```

- `wrap_index(i, len)` returns `i` modulo `len` as a value in `0..len`: the `r` in `0..len` such
  that `i - r` is a multiple of `len` (meant mathematically, as if computed with integers of
  unlimited size).
- `len` is between 1 and 1,000,000, and `i` can be any `i64`.
- It must not panic for any input allowed here.

### Examples

`wrap_index(-1, 5)` is `4`, and `wrap_index(12, 5)` is `2`.
