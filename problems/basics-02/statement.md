The code below does not compile. Fix it.

It contains two small number utilities.

```rust
pub fn fizz_label(n: u32) -> String
pub fn first_multiple_above(step: u64, limit: u64) -> u64
```

- `fizz_label(n)` returns `"FizzBuzz"` if `n` is a multiple of 15, otherwise `"Fizz"` if it is a
  multiple of 3, otherwise `"Buzz"` if it is a multiple of 5, and otherwise `n` written in decimal.
  `0` counts as a multiple of every number.
- `first_multiple_above(step, limit)` returns the smallest multiple of `step` that is **strictly
  greater** than `limit`. `step` is at least 1, and the answer always fits in a `u64`.

Do not change the signatures.

### Examples

- `fizz_label(10)` is `"Buzz"`, and `fizz_label(7)` is `"7"`.
- `first_multiple_above(7, 20)` is `21`, `first_multiple_above(5, 10)` is `15`, and
  `first_multiple_above(3, 0)` is `3`.
