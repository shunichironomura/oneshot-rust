The code below does not compile. Fix it.

The function below gives the FizzBuzz label of a number.

```rust
pub fn fizz_label(n: u32) -> String
```

- `fizz_label(n)` returns `"FizzBuzz"` if `n` is a multiple of 15, otherwise `"Fizz"` if it is a
  multiple of 3, otherwise `"Buzz"` if it is a multiple of 5, and otherwise `n` written in decimal.
- `0` counts as a multiple of every number.

Do not change the signature.

### Examples

- `fizz_label(10)` is `"Buzz"`, and `fizz_label(7)` is `"7"`.
