`?` converts the error with `From::from` before returning it, and core has the reflexive impl
`impl<T> From<T> for T`. That impl is what lets `ok_or(AppError::MissingComma)?` pass an `AppError`
through unchanged. The template adds `impl<E: Error> From<E> for AppError` for library errors, but
it covers **every** type that implements `Error`, and `AppError` is one of them. For `E = AppError`,
both impls would provide `From<AppError> for AppError`. Rust never lets two impls overlap (there is
no specialization on stable), so the compiler rejects the generic impl itself with E0119
("conflicting implementations"), even though no code ever converts an `AppError` into itself
through it.

Removing `impl Error for AppError` makes the overlap disappear, but it is a dead end, apart from
being forbidden by the statement. The `?` in `sum_pairs` boxes the error through std's
`impl<E: Error> From<E> for Box<dyn Error>`, and `downcast_ref::<AppError>()` needs
`AppError: Error` as well. A type cannot both implement `Error` and convert from every `Error`.

So name the error types that really occur. `parse` returns a `ParseIntError` for the `i32` and a
`ParseFloatError` for the `f64`. With one `From` impl for each, the rest of the code works
unchanged.

The same conflict is why `Box<dyn Error>` does not implement `Error` itself, and why
`anyhow::Error`, which converts from every error type, does not implement it either.

### Alternatives

Convert at the call site with `map_err` instead of a `From` impl. The turbofish is required here.
Without `::<i32>`, the compiler checks the closure body `e.to_string()` while it checks the
`map_err` call, when nothing has fixed the target type of `parse` yet. The type of `e` is unknown,
and compilation stops with "type annotations needed" (E0282). Annotating the variable, as in
`let a: i32 = ...?;`, does not help, because that type reaches `parse` only later, through the `?`.

```rust
pub fn parse_pair(s: &str) -> Result<(i32, f64), AppError> {
    let (a, b) = s.split_once(',').ok_or(AppError::MissingComma)?;
    let a = a.trim().parse::<i32>().map_err(|e| AppError::Parse(e.to_string()))?;
    let b = b.trim().parse::<f64>().map_err(|e| AppError::Parse(e.to_string()))?;
    Ok((a, b))
}
```

To keep the "any error type" idea of the template, put it in a generic function instead of a
`From` impl. Coherence (the check that impls do not overlap) applies to impls only, so the function
is fine.

```rust
use std::str::FromStr;

fn parse_part<T>(s: &str) -> Result<T, AppError>
where
    T: FromStr,
    T::Err: fmt::Display,
{
    s.trim().parse::<T>().map_err(|e| AppError::Parse(e.to_string()))
}

pub fn parse_pair(s: &str) -> Result<(i32, f64), AppError> {
    let (a, b) = s.split_once(',').ok_or(AppError::MissingComma)?;
    Ok((parse_part(a)?, parse_part(b)?))
}
```

Finally, a bound that `AppError` does not meet also removes the overlap:
`impl<E: Error + Clone> From<E> for AppError` compiles, because `AppError` is not `Clone` and only
this crate could make it so (both parse errors are `Clone`). It is fragile, though: deriving
`Clone` for `AppError` later brings E0119 back, and errors that are not `Clone` lose the
conversion.
