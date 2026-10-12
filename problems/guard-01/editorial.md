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

Implementing `TryFrom` is enough. The standard library has the blanket implementation
`impl<T, U> TryInto<U> for T where U: TryFrom<T>`, so once `Guess: TryFrom<i32>` holds,
`value.try_into()` works too (the same way `From` gives you `Into`).

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
