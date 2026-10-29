Some values are best computed once, while the program is being compiled. Write two functions that
the compiler can run for you: they are both `const fn`, and the test harness uses them to initialize
`const` items.

```rust
pub const fn is_prime(n: u32) -> bool;
pub const fn first_primes<const N: usize>() -> [u32; N];

pub const PRIMES_10: [u32; 10] = first_primes::<10>();
```

- `is_prime(n)` tells whether `n` is a prime number. `0` and `1` are not prime. `n` can be any
  `u32`, including values close to `u32::MAX`, so do not try every divisor below `n`: trying
  divisors up to √n is enough.
- `first_primes::<N>()` returns the first `N` primes in ascending order. `first_primes::<0>()`
  returns an empty array.
- `PRIMES_10` holds the first ten primes (this line is already in the editor; keep it).
- Both functions must stay `const fn`. The harness evaluates them at compile time in items such
  as `const P: [u32; 5] = first_primes::<5>();`, and also calls them at runtime.

The bodies in the editor are placeholders that return wrong values.

### Examples

- `is_prime(97)` is `true`, and `is_prime(1)` is `false`.
- `first_primes::<5>()` is `[2, 3, 5, 7, 11]`, and `PRIMES_10` is
  `[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]`.
