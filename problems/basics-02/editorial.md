In Rust, `if` and `loop` are expressions: they have a value, just like `1 + 2`. Each slip here
comes from forgetting what that value is.

1. **All branches of an `if` must have the same type.** Three branches are `&str` and the last one is
   a `String`. Turn the literals into `String`s with `.to_string()` (or `String::from`). The `if`
   can then be the final expression of the function by itself. The compiler suggests borrowing the
   last branch as `&n.to_string()` instead, but then the whole `if` is a `&str`, and the error only
   moves to `label`, which must be returned as a `String`.
2. **`break` can carry a value.** A `loop` is an expression, and `break value` gives its value. A
   bare `break;` gives `()`, so it must be `break x;`.

### Alternatives

Both slips can be avoided altogether. A `match` on both remainders states the FizzBuzz rules
directly, and the next multiple can be computed without a loop: `limit / step * step` is the largest
multiple of `step` that is not greater than `limit`, so the answer is one `step` more.

```rust
pub fn fizz_label(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => n.to_string(),
    }
}

pub fn first_multiple_above(step: u64, limit: u64) -> u64 {
    (limit / step + 1) * step
}
```
