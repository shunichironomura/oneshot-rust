`s` is a comma-separated list of integers. Return their sum, or the error from the first item that
is not a valid integer.

```rust
use std::num::ParseIntError;

pub fn sum_csv(s: &str) -> Result<i64, ParseIntError>
```

- Each item may have whitespace around it, which is ignored. An item may start with `+` or `-`.
- An empty item (such as the middle of `"1,,2"`) is an error.
- If `s` is empty or contains only whitespace, the sum is `0`.
- The error must be the one that parsing the first invalid item as an `i64` (after trimming)
  produces.

### Examples

| Input | Output |
| --- | --- |
| `"1, 2, 3"` | `Ok(6)` |
| `"4,x,5"` | `Err(..)`, the error from parsing `"x"` |
