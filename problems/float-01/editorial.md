`f64` implements `PartialOrd` but not `Ord`, because NaN is neither smaller than, equal to, nor
larger than any value, not even itself. `Iterator::max`, `Iterator::min` and `slice::sort` require
`Ord`, so calling them on `f64` values does not compile (E0277). The common workaround
`a.partial_cmp(b).unwrap()` compiles, but panics as soon as it compares a NaN.

The solution first drops the NaN readings with `filter(|x| !x.is_nan())`. It then orders the rest
with `f64::total_cmp`, a total order on all `f64` values, which can be passed directly to `max_by`,
`min_by` and `sort_by`. It orders `-∞` and `∞` as expected. `total_cmp` orders NaN too (a
positive NaN after `∞`, a negative NaN before `-∞`), so the filter is still needed: without it,
a positive NaN would become the maximum and a negative NaN the minimum.

`spread` reuses the maximum and the minimum. `?` returns `None` early when there is no reading.

In a hand-written loop, starting the maximum at `0.0` is wrong when every reading is negative.
Starting at `f64::MIN` gives a wrong answer for a single `-∞` reading, and starting at
`f64::NEG_INFINITY` cannot tell "no reading" from "a single `-∞` reading". Keeping an
`Option<f64>` as the running maximum avoids all of these.

### Alternatives

`f64::max` and `f64::min` ignore NaN: if one argument is NaN, they return the other one. Folding
from NaN as "no value yet" therefore gives the maximum, and the result is NaN only if there is no
reading.

```rust
pub fn max_reading(v: &[f64]) -> Option<f64> {
    let max = v.iter().copied().fold(f64::NAN, f64::max);
    if max.is_nan() { None } else { Some(max) }
}
```

Once the NaN readings are gone, `partial_cmp` always returns `Some`, so `unwrap` is safe there.

```rust
pub fn sorted_readings(v: &[f64]) -> Vec<f64> {
    let mut readings: Vec<f64> = v.iter().copied().filter(|x| !x.is_nan()).collect();
    readings.sort_by(|a, b| a.partial_cmp(b).unwrap());
    readings
}
```

`spread` can also take both ends of the sorted readings.

```rust
pub fn spread(v: &[f64]) -> Option<f64> {
    let sorted = sorted_readings(v);
    Some(sorted.last()? - sorted.first()?)
}
```
