Implement `From`, and `Into` comes for free: the standard library has
`impl<T, U: From<T>> Into<U> for T`. That is why `Celsius(-40.0).into()` works as soon as
`From<Celsius> for Fahrenheit` exists. It also has the reflexive `impl<T> From<T> for T`, so
`Celsius` is already `Into<Celsius>`: `is_freezing(Celsius(0.5))` and
`average(vec![Celsius(10.0)])` work with no extra code, and writing `impl From<Celsius> for Celsius`
yourself is rejected as a conflicting implementation (E0119).

The blanket impl only goes one way. With nothing but the bound `T: Into<Celsius>`,
`Celsius::from(t)` is an error (E0277, `Celsius: From<T>` is not satisfied). Call `t.into()` and
name the target type: `let c: Celsius = t.into();`. The target type matters even more on concrete
types: `Fahrenheit(50.0).into().0` is E0282 (type annotations needed), because a `Fahrenheit` can
become a `Celsius` or, through the reflexive impl, a `Fahrenheit`. (On the generic `t`,
`t.into().0` happens to compile, because the bound `Into<Celsius>` is the only conversion the
compiler knows for it.) In `warmest`, `Celsius::from(f)` avoids the question. It takes a
`Fahrenheit`, not a `&Fahrenheit`, hence the `|&f|` pattern.

`f64` is `PartialOrd` but not `Ord` (NaN cannot be ordered), so `Iterator::max` does not compile on
floats or on the newtypes. Use `max_by` with `f64::total_cmp`, or `reduce(f64::max)`. Folding from
`0.0` gives a wrong answer when every reading is below zero.

In `average`, `Celsius` does not implement `Sum`, so add up the inner `f64` values. `into_iter()`
consumes the `Vec`, so read `len()` first, and turn it into an `f64` with `as` before dividing.
Handle the empty list separately: `0.0 / 0.0` is NaN, not `None`.

### Alternatives

`warmest` can find the highest value in Fahrenheit and convert only that one. The closure returns
`Fahrenheit(f).into()`, and the return type of `warmest` tells the compiler that the target is
`Celsius`.

```rust
pub fn warmest(readings: &[Fahrenheit]) -> Option<Celsius> {
    readings
        .iter()
        .map(|f| f.0)
        .reduce(f64::max)
        .map(|f| Fahrenheit(f).into())
}
```

`average` with a loop, and `is_freezing` with the target type written on the trait:

```rust
pub fn is_freezing(t: impl Into<Celsius>) -> bool {
    Into::<Celsius>::into(t).0 <= 0.0
}

pub fn average<T: Into<Celsius>>(readings: Vec<T>) -> Option<Celsius> {
    let count = readings.len();
    let mut total = 0.0;
    for r in readings {
        let c: Celsius = r.into();
        total += c.0;
    }
    if count == 0 {
        None
    } else {
        Some(Celsius(total / count as f64))
    }
}
```
