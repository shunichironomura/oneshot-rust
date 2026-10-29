`Result` implements `FromIterator`. Collecting an iterator of `Result<i32, E>` into
`Result<Vec<i32>, E>` gives `Ok` with all the values, or the first `Err`; it stops there and does
not look at the remaining items. The return type tells `collect` what to build. `parse` does not
skip whitespace, which is exactly what the rules ask for.

Passing `str::parse::<i32>` to `map` directly does not compile (E0631): the iterator yields
`&&str`, but that function takes a `&str`. The closure works because a method call dereferences its
receiver automatically (adding `.copied()` before `map` works too).

### Alternatives

A plain loop works too: `?` inside the loop returns at the first invalid item.

```rust
use std::num::ParseIntError;

pub fn parse_all(items: &[&str]) -> Result<Vec<i32>, ParseIntError> {
    let mut values = Vec::new();
    for item in items {
        values.push(item.parse::<i32>()?);
    }
    Ok(values)
}
```
