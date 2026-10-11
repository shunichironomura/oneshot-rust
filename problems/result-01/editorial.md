`str::parse::<i64>()` returns a `Result<i64, ParseIntError>`. The `?` operator unwraps an `Ok`, or
returns the `Err` from the function right away, so the first invalid item decides the error and the
rest are never parsed.

`parse` does not skip whitespace, so `" 2"` fails unless the item is `trim`med first. It does accept
a leading `+`.

`"".split(',')` yields one empty item, not zero items, so an empty input would be an error without
the check at the top.

### Alternatives

`Result` implements `Sum`: summing an iterator of `Result<i64, E>` gives `Ok(total)`, or the first
`Err` it meets.

```rust
use std::num::ParseIntError;

pub fn sum_csv(s: &str) -> Result<i64, ParseIntError> {
    if s.trim().is_empty() {
        return Ok(0);
    }
    s.split(',').map(|item| item.trim().parse::<i64>()).sum()
}
```
