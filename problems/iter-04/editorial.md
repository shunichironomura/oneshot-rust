`iter::successors` builds a sequence in which each item is computed from the previous one, and ends
when the closure returns `None`. `checked_mul` returns `None` exactly when the product would
overflow, so the sequence ends after the largest power that fits. With `p * base`, the overflow
panics ("attempt to multiply with overflow") in a debug build such as the judge's, and silently
wraps around in a release build. Write the start as `Some(1u64)`: with `Some(1)`, the type of `p` is
not known yet when the closure calls `checked_mul` on it, and that is a compile error. Floating-point
logarithms are no way out either: `u64::MAX as f64` rounds up to exactly 2⁶⁴, so for a base such as
`2` or `16` the logarithm comes out as exactly `64` or `16`, and 2⁶⁴, which does not fit, is
counted too.

### Alternatives

`checked_pow` with `map_while` (a `map` that stops at the first `None`) works too. The exponent
`0..` is a `u32`, the type `checked_pow` takes.

```rust
pub fn powers_of(base: u64) -> Vec<u64> {
    (0..).map_while(|e| base.checked_pow(e)).collect()
}
```

Or compute the largest exponent first with the exact integer logarithm `ilog`.

```rust
pub fn powers_of(base: u64) -> Vec<u64> {
    (0..=u64::MAX.ilog(base)).map(|e| base.pow(e)).collect()
}
```
