Temperatures are stored as two newtypes, `Celsius` and `Fahrenheit` (both are already in the
editor). Implement the conversions between them and three functions that work with readings.

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Celsius(pub f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fahrenheit(pub f64);

impl From<Fahrenheit> for Celsius { /* to implement */ }
impl From<Celsius> for Fahrenheit { /* to implement */ }

pub fn is_freezing(t: impl Into<Celsius>) -> bool
pub fn average<T: Into<Celsius>>(readings: Vec<T>) -> Option<Celsius>
pub fn warmest(readings: &[Fahrenheit]) -> Option<Celsius>
```

- `Celsius::from(Fahrenheit(f))` is `Celsius((f - 32.0) * 5.0 / 9.0)`, and
  `Fahrenheit::from(Celsius(c))` is `Fahrenheit(c * 9.0 / 5.0 + 32.0)`. The tests also convert
  with `.into()`, in both directions.
- `is_freezing` returns `true` if the temperature, converted to Celsius, is `0.0` or below. It is
  called with both `Celsius` and `Fahrenheit` values.
- `average` converts every reading to Celsius and returns their sum divided by their count. It
  returns `None` if `readings` is empty. It is called with a `Vec<Celsius>` and with a
  `Vec<Fahrenheit>`.
- `warmest` returns the highest reading, converted to Celsius. It returns `None` if `readings` is
  empty.
- No value is NaN, and every value in the tests converts exactly, so results are compared with `==`.

### Examples

```rust
Celsius::from(Fahrenheit(212.0))                 // Celsius(100.0)
let f: Fahrenheit = Celsius(-40.0).into();       // Fahrenheit(-40.0)

is_freezing(Fahrenheit(32.0))                    // true
is_freezing(Celsius(0.5))                        // false

average(vec![Fahrenheit(32.0), Fahrenheit(212.0)])  // Some(Celsius(50.0))
average(vec![Celsius(10.0), Celsius(20.0)])         // Some(Celsius(15.0))
warmest(&[Fahrenheit(50.0), Fahrenheit(212.0), Fahrenheit(32.0)])  // Some(Celsius(100.0))
```
