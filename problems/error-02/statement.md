Add up the integers in a small text file. When something is wrong, return the first problem as a
boxed error.

```rust
use std::error::Error;

pub fn total(input: &str) -> Result<i64, Box<dyn Error>>
```

- Go through the lines of `input` (as `str::lines` splits them) in order. Trim the whitespace at
  both ends of each line.
- Skip a line if, after trimming, it is empty or starts with `#`.
- Every other line must be a single integer: parse it with `parse::<i64>()`. Return `Ok` with the
  sum of these integers.
- Stop at the first problem and return an error for it:
  - A line does not parse: return the `ParseIntError` that `parse` produced, unchanged. The tests
    check that `err.downcast_ref::<ParseIntError>()` is `Some` and equal to that error.
  - Adding an integer takes the running sum outside the range of `i64`: return an error whose
    `to_string()` is `overflow`. This is an error even if later integers would bring the sum back
    into range.
- If there is no problem but also no integer at all, return an error whose `to_string()` is
  `no numbers`. A sum of `0` is not an error.

### Examples

| Input | Result |
| --- | --- |
| `"1\n# c\n\n 2 "` | `Ok(3)` |
| `"1\nx"` | `Err(..)`, a `ParseIntError` displayed as `invalid digit found in string` |
| `""` | `Err(..)`, displayed as `no numbers` |
