Implement `powers_of`, which lists the powers of a number for as long as they fit in a `u64`.

```rust
pub fn powers_of(base: u64) -> Vec<u64>
```

- `powers_of` returns the powers of `base` in increasing order, `1, base, base², ...`, up to the
  largest power that fits in a `u64`.
- `base` is at least `2`.

### Examples

- `powers_of(10)` has 20 elements, from `1` to `10_000_000_000_000_000_000` (10¹⁹).
- The last element of `powers_of(2)` is `1 << 63` (2⁶³).
