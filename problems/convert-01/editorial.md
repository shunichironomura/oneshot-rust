`Mon = 1` sets the first discriminant, and each following variant without one gets the previous value
plus 1, so `Sun` is 7. A field-less enum can be cast to an integer with `as`, which makes `number`
simply `self as u8`. The cast only goes one way: `n as Weekday` is a compile error ("non-primitive
cast"), because most `u8` values are not a day. A conversion that can fail is what `TryFrom` is for:
it returns a `Result`, and a `match` on the number is the plainest way to write it. For the same
reason `next` cannot be `(self as u8 + 1) as Weekday`. A `match` lists the seven successors and makes
the wrap from `Sun` to `Mon` explicit.

Implementing the standard traits plugs `Weekday` into existing APIs. `impl TryFrom<u8> for Weekday`
also gives `u8: TryInto<Weekday>` through a blanket impl in `std`, and `TryFrom` and `TryInto` have
been in the prelude since the 2021 edition, so `0u8.try_into()` works without a `use`. In the same
way, `impl FromStr` is what makes `"fri".parse::<Weekday>()` work. Note that the error type is
called `Error` in `TryFrom` but `Err` in `FromStr`.

In `from_str`, `s.to_ascii_lowercase()` returns a new `String`. String literal patterns match a
`&str`, not a `String`, so match on `.as_str()`. The original `s` is untouched, so the error can keep
the text exactly as given: `DayError::Unknown(s.to_string())` (the `&str` has to become an owned
`String` first).

### Alternatives

Table-driven conversions: one array of days, used by `try_from` (index `n - 1`) and by `from_str`
(next to an array of names, compared with `eq_ignore_ascii_case`). `next` can then go through the
numbers: `% 7` turns `Sun`'s 7 into 0, so the result is always in `1..=7` and the `unwrap` never
fails.

```rust
const ALL: [Weekday; 7] = [
    Weekday::Mon,
    Weekday::Tue,
    Weekday::Wed,
    Weekday::Thu,
    Weekday::Fri,
    Weekday::Sat,
    Weekday::Sun,
];

const NAMES: [&str; 7] = ["mon", "tue", "wed", "thu", "fri", "sat", "sun"];

impl Weekday {
    pub fn next(self) -> Weekday {
        // 1..=6 become 2..=7, and 7 (Sun) becomes 0 + 1 = 1 (Mon).
        Weekday::try_from(self.number() % 7 + 1).unwrap()
    }
    // number as in the solution
}

impl TryFrom<u8> for Weekday {
    type Error = DayError;

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        match n {
            1..=7 => Ok(ALL[usize::from(n - 1)]),
            _ => Err(DayError::OutOfRange(n)),
        }
    }
}

impl FromStr for Weekday {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for (day, name) in ALL.into_iter().zip(NAMES) {
            if s.eq_ignore_ascii_case(name) {
                return Ok(day);
            }
        }
        Err(DayError::Unknown(s.to_string()))
    }
}
```
