The code below does not compile. **Without looking at the error message**, predict where the compiler
complains and fix it.

Each line of a small CSV file holds three columns: an id, an age and a score. `validate_row` checks
that a line can be read (the values themselves are not needed), and `sum_ids` adds up the ids of a
list of lines. `BadField` describes the column that could not be read (it is already in the editor).

```rust
#[derive(Debug, PartialEq)]
pub struct BadField {
    pub column: usize,
    pub text: String,
}

pub fn validate_row(line: &str) -> Result<(), BadField>;
pub fn sum_ids(lines: &[&str]) -> Result<u64, BadField>;
```

- A line is split at every `,`. Columns are numbered from 0: column 0 is the id, column 1 the age
  and column 2 the score. Columns after column 2 are ignored, and a missing column counts as the
  empty text `""`.
- The text of each column is trimmed with `str::trim` and then parsed with `str::parse`: the id as
  a `u32`, the age as a `u8` and the score as an `f64`.
- `validate_row` checks columns 0, 1 and 2 in this order. For the first column that does not parse,
  it returns `Err(BadField { column, text })`, where `text` is the trimmed text of that column.
  If all three parse, it returns `Ok(())`.
- `sum_ids` checks the lines in order with the rules of `validate_row`. If a line is invalid, it
  returns the error of the first invalid line. Otherwise it returns `Ok` with the sum of all ids,
  which may exceed `u32::MAX` (`Ok(0)` for an empty list).

Do not change the public signatures. The private helper `field` may be changed or removed.

### Examples

- `validate_row("7, 30, 1.5")` is `Ok(())`.
- `validate_row("7, 300, 1.5")` is `Err(BadField { column: 1, text: "300".to_string() })`, because
  300 does not fit in a `u8`.
- `validate_row("7")` is `Err(BadField { column: 1, text: "".to_string() })`: the age is missing.
- `sum_ids(&["7, 30, 1.5", "35, 41, 0.25"])` is `Ok(42)`.
