Some values are best computed once, while the program is being compiled. Write four functions that
the compiler can run for you: they are all `const fn`, and the test harness uses them to initialize
`const` items.

```rust
pub const fn gcd(a: u64, b: u64) -> u64;
pub const fn is_prime(n: u32) -> bool;
pub const fn first_primes<const N: usize>() -> [u32; N];
pub const fn weighted_sum(bytes: &[u8]) -> u32;

pub const PRIMES_10: [u32; 10] = first_primes::<10>();
```

- `gcd(a, b)` returns the greatest common divisor of `a` and `b`. If one argument is `0`, the
  result is the other one, so `gcd(0, 0)` is `0`. The arguments can be any `u64` values, and the
  result must come quickly even for values close to `u64::MAX`.
- `is_prime(n)` tells whether `n` is a prime number. `0` and `1` are not prime. `n` can be any
  `u32`, including values close to `u32::MAX`, so do not try every divisor below `n`: trying
  divisors up to √n is enough.
- `first_primes::<N>()` returns the first `N` primes in ascending order. `first_primes::<0>()`
  returns an empty array.
- `weighted_sum(bytes)` returns the sum of `(i + 1) * bytes[i]` over every index `i`: the first
  byte counts once, the second twice, and so on. An empty slice gives `0`. The harness only passes
  slices whose result fits in a `u32`.
- `PRIMES_10` holds the first ten primes (this line is already in the editor; keep it).
- Every function must stay a `const fn`. The harness evaluates them at compile time in items such
  as `const G: u64 = gcd(84, 36);` and `const P: [u32; 5] = first_primes::<5>();`, and also
  calls them at runtime.

The bodies in the editor are placeholders that return wrong values.

### Examples

- `gcd(84, 36)` is `12`.
- `is_prime(97)` is `true`, and `is_prime(1)` is `false`.
- `first_primes::<5>()` is `[2, 3, 5, 7, 11]`, and `PRIMES_10` is
  `[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]`.
- `weighted_sum(b"ab")` is `1 * 97 + 2 * 98 = 293` (`b'a'` is `97`, `b'b'` is `98`).
