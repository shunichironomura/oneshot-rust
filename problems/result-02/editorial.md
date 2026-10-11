Each function is a short chain of standard library methods, and each has one detail that is easy to
get wrong when you write it without a compiler at hand.

- `parse_all`: `Result` implements `FromIterator`. Collecting an iterator of `Result<i32, E>` into
  `Result<Vec<i32>, E>` gives `Ok` with all the values, or the first `Err`; it stops there and does
  not look at the remaining items. The return type tells `collect` what to build. `parse` does not
  skip whitespace, which is exactly what the rules ask for.
- `split_parsed`: iterating over `items`, a `&[&'a str]`, yields `&&'a str`. With `for item in items`
  and `let mut invalid = Vec::new();`, `invalid.push(item)` makes `invalid` a `Vec<&&str>`, which does
  not match the return type (E0308). The pattern in `for &item in items` copies the `&'a str` out of
  the slice (shared references are `Copy`). That reference borrows the strings for `'a`, not the
  slice, so the result stays usable after the slice is gone.
- `double_first`: `items.first()` is an `Option<&&str>`, and mapping it with `parse` gives an
  `Option<Result<i32, ParseIntError>>`. `transpose` swaps the two layers: `None` becomes `Ok(None)`,
  `Some(Ok(n))` becomes `Ok(Some(n))`, and `Some(Err(e))` becomes `Err(e)`, which `?` returns. Then
  double the value as an `i64`. On an `i32`, `2 * n` overflows for `2147483647` and panics, because
  the judge checks for overflow as debug builds do, and `(2 * n) as i64` converts too late.
  `2 * i64::from(n)` is safe (so is `2 * n as i64`, because `as` binds tighter than `*`). Indexing
  with `items[0]` would panic for an empty slice.
- `sum_all`: `Option` implements `Sum`, and `xs.iter().copied().sum::<Option<u32>>()` returns `None`
  when an element is `None`. But it adds with the ordinary `+`, which panics on overflow. `try_fold`
  stops at the first `None` that its closure returns: `x?` returns `None` for a missing element, and
  `checked_add` returns `None` on overflow.

### Alternatives

Plain loops work too: `?` inside the loop returns at the first problem.

```rust
use std::num::ParseIntError;

pub fn parse_all(items: &[&str]) -> Result<Vec<i32>, ParseIntError> {
    let mut values = Vec::new();
    for item in items {
        values.push(item.parse::<i32>()?);
    }
    Ok(values)
}

pub fn double_first(items: &[&str]) -> Result<Option<i64>, ParseIntError> {
    match items.first() {
        None => Ok(None),
        Some(item) => {
            let n: i32 = item.parse()?;
            Ok(Some(2 * i64::from(n)))
        }
    }
}

pub fn sum_all(xs: &[Option<u32>]) -> Option<u32> {
    let mut total: u32 = 0;
    for &x in xs {
        total = total.checked_add(x?)?;
    }
    Some(total)
}
```

`partition` splits an iterator in two with a predicate. Replacing each error with the item itself
(`map_err(|_| item)`) turns every element into a `Result<i32, &str>`. After the split, every `unwrap`
and `unwrap_err` is known to succeed.

```rust
pub fn split_parsed<'a>(items: &[&'a str]) -> (Vec<i32>, Vec<&'a str>) {
    let (valid, invalid): (Vec<_>, Vec<_>) = items
        .iter()
        .map(|&item| item.parse::<i32>().map_err(|_| item))
        .partition(Result::is_ok);
    (
        valid.into_iter().map(Result::unwrap).collect(),
        invalid.into_iter().map(Result::unwrap_err).collect(),
    )
}
```

`Option` implements `FromIterator` as well: collecting into `Option<Vec<u32>>` gives `None` if an
element is missing. `u32::checked_add` can then be passed to `try_fold` as it is.

```rust
pub fn sum_all(xs: &[Option<u32>]) -> Option<u32> {
    let values: Option<Vec<u32>> = xs.iter().copied().collect();
    values?.into_iter().try_fold(0, u32::checked_add)
}
```
