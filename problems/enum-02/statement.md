A spreadsheet row arrives as one line of comma-separated text. Turn it into typed cells with the
`Cell` enum below (it is already in the editor), then compute two things from a row.

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
    Int(i64),
    Float(f64),
    Text(String),
    Empty,
}

pub fn parse_row(line: &str) -> Vec<Cell>
pub fn numeric_sum(row: &[Cell]) -> f64
pub fn texts(row: &[Cell]) -> Vec<&str>
```

- `parse_row` splits `line` at every `,` and returns one cell per field, in order. Even an empty
  line has one field: a line with *n* commas always gives *n* + 1 cells.
- Each field is trimmed first: leading and trailing whitespace is removed, as `str::trim` does.
  The trimmed field then becomes the first of the following that applies, checked from top to
  bottom.

| Trimmed field | Cell |
| --- | --- |
| is empty | `Empty` |
| `field.parse::<i64>()` returns `Ok(n)` | `Int(n)` |
| `field.parse::<f64>()` returns `Ok(x)` | `Float(x)` |
| anything else | `Text` holding the trimmed field |

These two standard parsers alone decide what is a number, including the less common forms they
accept, such as `.5` or `inf`. The tests contain no field that parses to NaN.

- `numeric_sum` returns the sum of the values of all `Int` and `Float` cells. Each value is
  converted to `f64` before it is added, so large integers cannot overflow `i64`. The sum is `0.0`
  if the row has no numbers.
- `texts` returns the strings of the `Text` cells in order, as slices borrowed from `row`.

### Examples

```rust
let row = parse_row("3, +5, 1e3, -0.5,  blue sky , ");
// [Int(3), Int(5), Float(1000.0), Float(-0.5), Text("blue sky"), Empty]

numeric_sum(&row) // 1007.5
texts(&row)       // ["blue sky"]
parse_row("")     // [Empty]
```
