In Rust, `if` is an expression: it has a value, just like `1 + 2`, and **all its branches must have
the same type**. Three branches are `&str` and the last one is a `String`. Turn the literals into
`String`s with `.to_string()` (or `String::from`). The `if` can then be the final expression of the
function by itself.

The compiler suggests borrowing the last branch as `&n.to_string()` instead, but then the whole `if`
is a `&str`, and the error only moves to `label`, which must be returned as a `String`.

### Alternatives

A `match` on both remainders states the FizzBuzz rules directly.

```rust
pub fn fizz_label(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => n.to_string(),
    }
}
```
