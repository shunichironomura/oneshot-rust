There are several problems, and the compiler does not report them all at once: type errors come first,
and the missing `mut` is reported only after the types are fixed.

- `sum += values[i]` adds an `i32` to an `i64`. Rust never converts integer types implicitly, so the
  value has to be widened with `i64::from(...)` (or `as i64`). The same applies to `sum -= ...`.
- `sum / window` divides an `i64` by a `usize`, which is also a mismatch. Even with matching integer
  types, integer division would drop the fraction, and `averages` is a `Vec<f64>`. Convert both
  sides to `f64` first: `sum as f64 / window as f64`.
- `sum` is changed in the loop, so it must be declared `let mut`.

Keep `sum` as an `i64`. Changing it to `i32` would compile, but adding two values near `i32::MAX`
overflows, which panics in debug builds.

Indexing with `values[i]` is fine as it is: `i` comes from `0..values.len()`, so it is a `usize`.

### Alternatives

`i64::from` is a lossless conversion and is rejected by the compiler if it ever could lose
information, which makes it safer than `as` for widening. `as` is still the usual way to get an
`f64`. A version using `windows`, which yields every run of `window` consecutive elements:

```rust
pub fn moving_average(values: &[i32], window: usize) -> Vec<f64> {
    if window == 0 {
        return Vec::new();
    }
    values
        .windows(window)
        .map(|w| w.iter().map(|&x| i64::from(x)).sum::<i64>() as f64 / window as f64)
        .collect()
}
```

`windows` panics for a size of `0`, hence the check; it yields nothing when `window` is larger than
the slice. It recomputes each sum, which is fine for short inputs.
