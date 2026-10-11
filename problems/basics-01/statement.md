The code below does not compile. **Without looking at the error message**, predict where the compiler
complains and fix it.

It contains three small number utilities.

```rust
pub fn digital_root(n: u64) -> u64
pub fn fizz_label(n: u32) -> String
pub fn first_multiple_above(step: u64, limit: u64) -> u64
```

- `digital_root(n)` repeatedly replaces `n` by the sum of its decimal digits until it is less than
  10, and returns that number. A number that is already less than 10 (including `0`) is returned as
  it is.
- `fizz_label(n)` returns `"FizzBuzz"` if `n` is a multiple of 15, otherwise `"Fizz"` if it is a
  multiple of 3, otherwise `"Buzz"` if it is a multiple of 5, and otherwise `n` written in decimal.
  `0` counts as a multiple of every number.
- `first_multiple_above(step, limit)` returns the smallest multiple of `step` that is **strictly
  greater** than `limit`. `step` is at least 1, and the answer always fits in a `u64`.

Do not change the signatures.

### Examples

- `digital_root(9875)` is `2` (9875 → 29 → 11 → 2).
- `fizz_label(10)` is `"Buzz"`, and `fizz_label(7)` is `"7"`.
- `first_multiple_above(7, 20)` is `21`, `first_multiple_above(5, 10)` is `15`, and
  `first_multiple_above(3, 0)` is `3`.
