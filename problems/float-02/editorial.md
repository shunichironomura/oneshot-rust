`to_int_unchecked` converts a float to an integer without any check. Its contract says that the value
must not be NaN or infinite, and that it must fit in the target type once the fractional part is
dropped; anything else is undefined behavior. The `SAFETY` comment relies on samples being in
`0.0..=1.0`, but the statement says they can be anything. In the example, `1.5` gives the level
`383`, which does not fit in a `u8`, and Miri stops right there.

A plain `as` cast already does exactly what the statement asks for. Since Rust 1.45, an `as` cast
from a float to an integer **saturates**: a value below the smallest value of the target type
becomes that value (`0` for `u8`), a value above the largest becomes the largest (`255`), and NaN
becomes `0`. The fractional part is cut off, so round first. An `as` cast never causes UB, so no
`unsafe` is needed. `to_int_unchecked` is only for code that has already proven the range and wants
to skip those checks.

Keeping `to_int_unchecked` and adding `.clamp(0.0, 255.0)` is not enough: `f32::clamp` returns NaN
when its input is NaN, so a NaN sample is still UB. (`f32::max` and `f32::min` behave differently:
when one argument is NaN, they return the other one.)

### Alternatives

Spell out every case with comparisons, without relying on how `as` treats out-of-range values. Every
comparison with NaN is `false`, so NaN needs its own check.

```rust
pub fn to_levels(samples: &[f32]) -> Vec<u8> {
    let mut levels = Vec::with_capacity(samples.len());
    for &s in samples {
        let level = (s * 255.0).round();
        let level = if level.is_nan() || level <= 0.0 {
            0
        } else if level >= 255.0 {
            255
        } else {
            level as u8
        };
        levels.push(level);
    }
    levels
}
```

`to_int_unchecked` can stay if every value that reaches it is in range. Handle NaN first, then
clamp.

```rust
pub fn to_levels(samples: &[f32]) -> Vec<u8> {
    samples
        .iter()
        .map(|&s| {
            let level = (s * 255.0).round();
            if level.is_nan() {
                return 0;
            }
            // SAFETY: `level` is not NaN, so after clamping it is a whole number in 0.0..=255.0.
            unsafe { level.clamp(0.0, 255.0).to_int_unchecked::<u8>() }
        })
        .collect()
}
```
