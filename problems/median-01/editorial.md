`v` is a borrowed slice (`&[i32]`), so `v.sort()` does not compile: sorting needs `&mut`. Copy the
values into a `Vec` you own with `to_vec()` and sort that. `sort_unstable` is enough for integers,
since equal values cannot be told apart.

For an even length, do not add the two middle values as `i32`. `i32::MAX + i32::MAX` overflows,
which panics in debug builds, and integer division would drop the `.5` anyway. Convert each value
to `f64` first: an `f64` holds every `i32` exactly, and so does the sum of two of them (at most 2^32
in absolute value, far below 2^53, the limit for exact integers). Halving it is exact too.
`f64::from` is a lossless conversion; `as f64` gives the same result here.

### Alternatives

The middle values can also be added the middle values as `i64`, where the sum of two `i32` values always fits.
Converting that sum to `f64` is exact. `f64::from` does not accept an `i64` (not every `i64` fits in
an `f64` exactly), so this conversion needs `as`. Do not divide by 2 before converting: integer
division turns `-1 / 2` into `0`.

```rust
pub fn median(v: &[i32]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    let mut sorted = v.to_vec();
    sorted.sort_unstable();
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 1 {
        Some(f64::from(sorted[mid]))
    } else {
        let sum = i64::from(sorted[mid - 1]) + i64::from(sorted[mid]);
        Some(sum as f64 / 2.0)
    }
}
```
