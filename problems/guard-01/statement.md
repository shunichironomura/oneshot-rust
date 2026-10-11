In the guessing game, a guess must be a number from 1 to 100. Instead of checking that again
everywhere a guess is used, build a type that can only hold valid values. `Guess` keeps its value in
a private field, so the functions below are the only way to create one, and every `Guess` is valid.
The `Guess` struct and the `GuessError` enum are already in the editor. Implement the methods, the
`TryFrom` conversion and `parse_guess`.

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
    pub fn compare(&self, secret: i32) -> Ordering;
}

impl TryFrom<i32> for Guess {
    type Error = GuessError;
    fn try_from(value: i32) -> Result<Guess, GuessError>;
}

pub fn parse_guess(input: &str) -> Result<Guess, String>
```

- A value is valid if it is between 1 and 100, inclusive.
- `Guess::new(value)` returns a `Guess` holding `value` if it is valid. Otherwise it panics with one
  of these messages, where `{value}` is the given value in decimal. The tests catch the panic and
  compare its message exactly.
  - If `value` is less than 1: `Guess value must be greater than or equal to 1, got {value}.`
  - If `value` is greater than 100: `Guess value must be less than or equal to 100, got {value}.`
- `value()` returns the value held by the guess.
- `compare(secret)` tells how the guess compares to `secret`: `Ordering::Less` if the guess is
  smaller than `secret`, `Ordering::Equal` if they are equal, and `Ordering::Greater` if the guess is
  larger. `secret` can be any `i32`, even one outside 1 to 100.
- `Guess::try_from(value)` (and so `value.try_into()`) makes the same check without panicking. It
  returns `Ok` of the guess if `value` is valid, `Err(GuessError::TooSmall(value))` if `value` is
  less than 1, and `Err(GuessError::TooLarge(value))` if it is greater than 100.
- `parse_guess(input)` reads a guess typed by the player:
  - First, whitespace is removed from both ends of `input`, as `str::trim` does.
  - If the rest is not accepted by `str::parse::<i32>`, the result is `Err` of `"not a number"`.
    This includes the empty string, text such as `"4.5"` or `"4 2"`, and numbers that do not fit in
    an `i32`, such as `"3000000000"`.
  - If it is a number but not a valid value, the result is `Err` of `"out of range"`.
  - Otherwise the result is `Ok` of the guess.

### Examples

```rust
Guess::new(42).value()      // 42
Guess::new(30).compare(50)  // Ordering::Less (the guess 30 is smaller than the secret 50)
Guess::try_from(100)        // Ok of a guess with the value 100
Guess::try_from(0)          // Err(GuessError::TooSmall(0))

Guess::new(0)    // panics: "Guess value must be greater than or equal to 1, got 0."
Guess::new(200)  // panics: "Guess value must be less than or equal to 100, got 200."

parse_guess(" 42 ")  // Ok of a guess with the value 42
parse_guess("+7")    // Ok of a guess with the value 7
parse_guess("0")     // Err("out of range".to_string())
parse_guess("abc")   // Err("not a number".to_string())
```
