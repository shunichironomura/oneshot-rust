Only part of Rust can run inside a `const fn`. A `for` loop is not part of it: it calls
`IntoIterator::into_iter` and `Iterator::next`, which are trait methods and not `const` (error
E0015). For the same reason, iterator chains such as `bytes.iter().map(...).sum()`, calling closures,
`Ord::max`, `std::array::from_fn` and the `?` operator are rejected. What remains is enough: `while`
and `loop`, `if` and `match`, arithmetic, `.len()`, indexing arrays and slices, and calls to other
`const fn`s, recursion included. So every loop becomes a `while` loop with a hand-written index.

`first_primes` starts from `[0; N]`, which works for a generic `N`, and fills it in place. Assigning
to the elements of a local array is allowed in a `const fn`. Check `count < N` before writing:
setting `primes[0] = 2` up front would index out of bounds when `N` is `0`.

Errors during constant evaluation are not runtime panics: an overflow, an out-of-bounds index or a
division by zero while computing a `const` item is a **compile error**. The usual suspect here is
`d * d <= n` in `is_prime`. For a prime close to `u32::MAX`, such as `4_294_967_291`, `d` reaches
`65536` (or `65537` when only odd `d` are tried), and `d * d` overflows `u32`. `d <= n / d` asks the
same question without overflowing. The compiler also gives up on constant evaluation that runs too
long (the `long_running_const_eval` lint), so trying every divisor below `n`, or a `gcd` that
subtracts instead of taking remainders, fails to compile on large inputs.

In `weighted_sum`, convert both factors to `u32` before multiplying, as in
`(i as u32 + 1) * bytes[i] as u32`. A product computed in `u8`, such as `(i as u8 + 1) * bytes[i]`,
overflows. A byte string literal such as `b"ab"` has type `&[u8; 2]` and coerces to `&[u8]`, also in
a `const` item.

### Alternatives

Recursion is allowed in a `const fn`, so `gcd` can follow Euclid's definition directly.

```rust
pub const fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}
```

`u32::isqrt` is a `const fn`. Computing the limit once also avoids the overflow.

```rust
pub const fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let limit = n.isqrt();
    let mut d = 2;
    while d <= limit {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}
```

Slice patterns work in a `const fn` too, so `weighted_sum` can take bytes off the front of the slice
with `while let` instead of indexing.

```rust
pub const fn weighted_sum(bytes: &[u8]) -> u32 {
    let mut rest = bytes;
    let mut weight = 1;
    let mut sum = 0;
    while let [first, tail @ ..] = rest {
        sum += weight * *first as u32;
        weight += 1;
        rest = tail;
    }
    sum
}
```
