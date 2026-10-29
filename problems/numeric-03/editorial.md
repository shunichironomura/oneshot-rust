`/` rounds toward zero, and `%` returns the matching remainder, which has the sign of the dividend:
`-1 % 5` is `-1`, which `as usize` turns into a huge number. `rem_euclid` chooses the remainder
that is never negative; for a positive divisor, that is a remainder in `0..len`.

Fixing up a negative remainder by adding `len` first, as in `(i + n) % n`, is not enough: it is
still negative for `i < -n`, and the judge builds your code with overflow checks, as debug builds
do, so `i64::MAX + n` panics ("attempt to add with overflow").

### Alternatives

Without `rem_euclid`, add `n` to the remainder instead of to `i`, and take the remainder again.

```rust
pub fn wrap_index(i: i64, len: usize) -> usize {
    let n = len as i64;
    // `i % n` is greater than `-n` and less than `n`, so adding `n` cannot overflow.
    ((i % n + n) % n) as usize
}
```
