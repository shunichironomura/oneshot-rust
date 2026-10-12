In the guessing game, a guess must be a number from 1 to 100. Instead of checking that again
everywhere a guess is used, build a type that can only hold valid values. `Guess` keeps its value in
a private field, so the functions below are the only way to create one, and every `Guess` is valid.
The `Guess` struct and the `GuessError` enum are already in the editor. Implement the methods and the
`TryFrom` conversion.

```rust
#[derive(Debug, PartialEq)]
pub struct Guess {
    value: i32,
}

#[derive(Debug, PartialEq)]
pub enum GuessError {
    TooSmall(i32),
    TooLarge(i32),
}

impl Guess {
    pub fn new(value: i32) -> Guess;
    pub fn value(&self) -> i32;
}

impl TryFrom<i32> for Guess {
    type Error = GuessError;
    fn try_from(value: i32) -> Result<Guess, GuessError>;
}
```

- A value is valid if it is between 1 and 100, inclusive.
- `Guess::new(value)` returns a `Guess` holding `value` if it is valid. Otherwise it panics with one
  of these messages, where `{value}` is the given value in decimal. The tests catch the panic and
  compare its message exactly.
  - If `value` is less than 1: `Guess value must be greater than or equal to 1, got {value}.`
  - If `value` is greater than 100: `Guess value must be less than or equal to 100, got {value}.`
- `value()` returns the value held by the guess.
- `Guess::try_from(value)` (and so `value.try_into()`) makes the same check without panicking. It
  returns `Ok` of the guess if `value` is valid, `Err(GuessError::TooSmall(value))` if `value` is
  less than 1, and `Err(GuessError::TooLarge(value))` if it is greater than 100.

### Examples

```rust
Guess::new(42).value()  // 42
Guess::try_from(100)    // Ok of a guess with the value 100
Guess::try_from(0)      // Err(GuessError::TooSmall(0))

Guess::new(0)    // panics: "Guess value must be greater than or equal to 1, got 0."
Guess::new(200)  // panics: "Guess value must be less than or equal to 100, got 200."
```
