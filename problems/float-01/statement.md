A weather station logs temperature readings as `f64` values. When the sensor fails, it logs NaN (not
a number) instead of a temperature. Implement three functions that summarize the readings.

```rust
pub fn max_reading(v: &[f64]) -> Option<f64>
pub fn sorted_readings(v: &[f64]) -> Vec<f64>
pub fn spread(v: &[f64]) -> Option<f64>
```

- All three functions ignore every NaN reading, including a negative NaN such as `-f64::NAN`.
- Infinities are ordinary readings: `f64::INFINITY` is larger than every other reading and
  `f64::NEG_INFINITY` is smaller.
- `max_reading` returns the largest reading, or `None` if there is none (the slice is empty or
  contains only NaN).
- `sorted_readings` returns the readings in ascending order. Equal readings are all kept.
- `spread` returns the largest reading minus the smallest one, computed with ordinary `f64`
  subtraction (a result too large for `f64` becomes `f64::INFINITY`). It is `None` if there is no
  reading.

### Examples

- `max_reading(&[1.5, f64::NAN, -2.0])` is `Some(1.5)`, and `max_reading(&[f64::NAN])` is `None`.
- `sorted_readings(&[3.0, f64::NAN, -1.0, 2.5])` is `[-1.0, 2.5, 3.0]`.
- `spread(&[4.0, f64::NAN, 1.5])` is `Some(2.5)`, and `spread(&[2.0])` is `Some(0.0)`.
