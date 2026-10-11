The judge builds your code with overflow checks, as debug builds do: an arithmetic operation whose
result does not fit in its type panics ("attempt to add with overflow") instead of silently
wrapping around as in a release build. The obvious formulas therefore fail at the edges:
`xs.iter().sum::<u8>()`, `(level + amount).min(100)`, `(items + per_page - 1) / per_page`,
`(a + b) / 2` and `(a - b).abs()` all panic for some allowed input. The integer types have a method
for each of these situations, which makes every function a one-liner:

- `wrapping_add` wraps around (modulo 256 for `u8`). `checked_add` returns `None` on overflow, and
  `try_fold` stops at the first `None`. `saturating_add` stops at `u8::MAX`, and `.min(100)` then
  applies the real cap.
- `/` and `%` round toward zero: `-1 / 10` is `0`, and `-1 % 5` is `-1`, which `as usize` turns
  into a huge number. `div_euclid` and `rem_euclid` choose the remainder that is never negative; for
  a positive divisor, that means division rounded down and a remainder in `0..len`.
- `div_ceil` divides rounding up (it is stable only for unsigned integers), `midpoint` averages two
  numbers without overflow, and `abs_diff` returns the distance in the unsigned type. That is
  needed because `i64::MAX - i64::MIN` does not fit in an `i64`.

Two near misses still panic. In `(a - b).unsigned_abs()`, the subtraction overflows before
`unsigned_abs` is reached. `xs.iter().map(|&x| Some(x)).sum::<Option<u8>>()` returns `None` only
when an item is `None`; the additions themselves are ordinary ones and panic on overflow.

### Alternatives

`sum_checked` can be a loop in which `?` returns `None` as soon as `checked_add` does, and
`std::num::Wrapping` makes `+` (and `sum`) wrap around.

```rust
use std::num::Wrapping;

pub fn sum_wrapping(xs: &[u8]) -> u8 {
    xs.iter().map(|&x| Wrapping(x)).sum::<Wrapping<u8>>().0
}

pub fn sum_checked(xs: &[u8]) -> Option<u8> {
    let mut total: u8 = 0;
    for &x in xs {
        total = total.checked_add(x)?;
    }
    Some(total)
}
```

Without the specialized methods, compute in a wider type or rearrange the formula so that no
intermediate value overflows. The wider type has to be wide enough, though: the bytes of a long
slice overflow even a `u16`.

```rust
pub fn charge(level: u8, amount: u8) -> u8 {
    (u16::from(level) + u16::from(amount)).min(100) as u8
}

pub fn wrap_index(i: i64, len: usize) -> usize {
    let n = len as i64;
    // `i % n` is greater than `-n` and less than `n`, so adding `n` cannot overflow.
    ((i % n + n) % n) as usize
}

pub fn bucket(x: i32, width: i32) -> i32 {
    let q = x / width;
    // A negative remainder means that `/` rounded up, toward zero.
    if x % width < 0 { q - 1 } else { q }
}

pub fn page_count(items: u64, per_page: u64) -> u64 {
    items / per_page + u64::from(items % per_page != 0)
}

pub fn middle(a: u64, b: u64) -> u64 {
    ((u128::from(a) + u128::from(b)) / 2) as u64
}

pub fn distance(a: i64, b: i64) -> u64 {
    (i128::from(a) - i128::from(b)).unsigned_abs() as u64
}
```

`middle` also works without a wider type as `a / 2 + b / 2 + (a & b & 1)`: halving each number
first loses one half from each odd number, and the last term adds the missing 1 when both are odd.
