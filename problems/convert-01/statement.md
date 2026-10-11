The days of the week are modeled by the `Weekday` enum below, and conversion failures by `DayError`
(both are already in the editor). Implement the three methods, the conversions from `u8` and from
text, and `parse_schedule`.

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
    pub fn is_weekend(self) -> bool;
}

impl TryFrom<u8> for Weekday { /* type Error = DayError; */ }
impl std::str::FromStr for Weekday { /* type Err = DayError; */ }

pub fn parse_schedule(s: &str) -> Result<Vec<Weekday>, DayError>
```

- `number` returns the day's number: `Mon` is 1, `Tue` 2, ..., `Sun` 7.
- `next` returns the following day. The week goes around: `Weekday::Sun.next()` is `Weekday::Mon`.
- `is_weekend` is `true` for `Sat` and `Sun`, and `false` for the other days.
- `Weekday::try_from(n)` converts the numbers 1 to 7 to `Mon` to `Sun` (the reverse of `number`).
  Any other `n` gives `Err(DayError::OutOfRange(n))`.
- `from_str` (used by `s.parse::<Weekday>()`) accepts exactly the three-letter English
  abbreviations `Mon`, `Tue`, `Wed`, `Thu`, `Fri`, `Sat` and `Sun`, ignoring ASCII case (`"mon"`,
  `"MON"` and `"mOn"` all give `Mon`). The text is not trimmed, and full names such as `"Monday"`
  are not accepted. Anything else gives `Err(DayError::Unknown(..))` holding the input exactly as it
  was given, with its original case and whitespace.
- `parse_schedule` reads a comma-separated list of days:
  - If `s` is empty or contains only whitespace, the result is `Ok` of an empty vector.
  - Otherwise `s` is split at every `,`. Each piece is trimmed (as `str::trim` does) and then
    parsed with `from_str`. The result is all the days, in order and with duplicates kept.
  - If a piece cannot be parsed, the result is the error for the **first** such piece, that is,
    `DayError::Unknown` of the trimmed piece. An empty piece (as in `"mon,,fri"` or `"mon,"`) is
    an error too, `Unknown("")`.

### Examples

- `Weekday::Thu.number()` is `4`, `Weekday::Fri.next()` is `Weekday::Sat`, and
  `Weekday::Sat.is_weekend()` is `true`.
- `Weekday::try_from(7)` is `Ok(Weekday::Sun)`. With `let r: Result<Weekday, _> = 0u8.try_into();`,
  `r` is `Err(DayError::OutOfRange(0))`.
- `"fri".parse::<Weekday>()` is `Ok(Weekday::Fri)`, and `"Funday".parse::<Weekday>()` is
  `Err(DayError::Unknown("Funday".to_string()))`.
- `parse_schedule("mon, FRI")` is `Ok(vec![Weekday::Mon, Weekday::Fri])`, and
  `parse_schedule("mon,xyz")` is `Err(DayError::Unknown("xyz".to_string()))`.
