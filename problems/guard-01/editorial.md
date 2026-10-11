`Guess` keeps its value in a private field, so code outside the module (the tests included) cannot
write `Guess { value: 0 }`. Every `Guess` is made by `new` or `try_from`, and both check the range,
so a function that receives a `Guess` can rely on its value without checking it again. `value()`
gives read-only access to the field.

The two constructors share one check: `try_from` reports the problem as a `GuessError`, and `new`
turns each error into its panic. Writing `Guess::try_from(value).unwrap()` (or `.expect(...)`) in
`new` would panic too, but with a message such as
``called `Result::unwrap()` on an `Err` value: TooSmall(0)``, not the required one. `panic!` formats
its message like `format!`, so `{value}` inserts the number. Keep the two messages straight: "less
than or equal to 100" goes with values that are too **large**. The Rust Book swaps exactly these two
messages in chapter 11 to show how `#[should_panic(expected = "...")]` catches the mistake.

`a.cmp(&b)` describes `a` relative to `b`, so `self.value.cmp(&secret)` is `Less` when the guess is
too small. `secret.cmp(&self.value)` gives the opposite answer.

In `parse_guess`, parse the trimmed text as an `i32` directly. `parse` accepts a leading `+` or `-`,
and it rejects numbers that overflow `i32`. Parsing as `u32` would wrongly call `"-5"` "not a
number" (and `u8` would do the same with `"300"`), while parsing as `i64` would call `"3000000000"`
"out of range". The `?` operator cannot turn a `ParseIntError` into a `String` on its own (there is
no such `From` impl), so `map_err` replaces the error with the message first.

### Alternatives

`assert!` takes a condition followed by a message, formatted like `panic!`, so `new` can check the
range directly. `try_from` stays as in the solution.

```rust
pub fn new(value: i32) -> Guess {
    assert!(value >= 1, "Guess value must be greater than or equal to 1, got {value}.");
    assert!(value <= 100, "Guess value must be less than or equal to 100, got {value}.");
    Guess { value }
}
```

`try_from` can `match` on ranges of values. The patterns `..=0`, `1..=100` and `101..` together
cover every `i32`, so the compiler accepts the `match` without a `_` arm.

```rust
fn try_from(value: i32) -> Result<Guess, GuessError> {
    match value {
        ..=0 => Err(GuessError::TooSmall(value)),
        1..=100 => Ok(Guess { value }),
        101.. => Err(GuessError::TooLarge(value)),
    }
}
```

`parse_guess` can also be written with `match` and an early `return`.

```rust
pub fn parse_guess(input: &str) -> Result<Guess, String> {
    let number = match input.trim().parse::<i32>() {
        Ok(number) => number,
        Err(_) => return Err(String::from("not a number")),
    };
    match Guess::try_from(number) {
        Ok(guess) => Ok(guess),
        Err(_) => Err(String::from("out of range")),
    }
}
```
