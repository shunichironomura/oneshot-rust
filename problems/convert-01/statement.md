The days of the week are modeled by the `Weekday` enum below, and conversion failures by `DayError`
(both are already in the editor). Implement the two methods and the conversions from `u8` and from
text.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weekday { Mon = 1, Tue, Wed, Thu, Fri, Sat, Sun }

#[derive(Debug, PartialEq, Eq)]
pub enum DayError {
    OutOfRange(u8),
    Unknown(String),
}

impl Weekday {
    pub fn number(self) -> u8;
    pub fn next(self) -> Weekday;
}

impl TryFrom<u8> for Weekday { /* type Error = DayError; */ }
impl std::str::FromStr for Weekday { /* type Err = DayError; */ }
```

- `number` returns the day's number: `Mon` is 1, `Tue` 2, ..., `Sun` 7.
- `next` returns the following day. The week goes around: `Weekday::Sun.next()` is `Weekday::Mon`.
- `Weekday::try_from(n)` converts the numbers 1 to 7 to `Mon` to `Sun` (the reverse of `number`).
  Any other `n` gives `Err(DayError::OutOfRange(n))`.
- `from_str` (used by `s.parse::<Weekday>()`) accepts exactly the three-letter English
  abbreviations `Mon`, `Tue`, `Wed`, `Thu`, `Fri`, `Sat` and `Sun`, ignoring ASCII case (`"mon"`,
  `"MON"` and `"mOn"` all give `Mon`). The text is not trimmed, and full names such as `"Monday"`
  are not accepted. Anything else gives `Err(DayError::Unknown(..))` holding the input exactly as it
  was given, with its original case and whitespace.

### Examples

- `Weekday::Thu.number()` is `4`, and `Weekday::Fri.next()` is `Weekday::Sat`.
- `Weekday::try_from(7)` is `Ok(Weekday::Sun)`. With `let r: Result<Weekday, _> = 0u8.try_into();`,
  `r` is `Err(DayError::OutOfRange(0))`.
- `"fri".parse::<Weekday>()` is `Ok(Weekday::Fri)`, and `"Funday".parse::<Weekday>()` is
  `Err(DayError::Unknown("Funday".to_string()))`.
