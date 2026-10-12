A spreadsheet row arrives as one line of comma-separated text. Turn it into typed cells with the
`Cell` enum below (it is already in the editor), then compute two things from a row.

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
    Int(i64),
    Text(String),
    Empty,
}

pub fn parse_row(line: &str) -> Vec<Cell>
pub fn numeric_sum(row: &[Cell]) -> i64
pub fn texts(row: &[Cell]) -> Vec<&str>
```

- `parse_row` splits `line` at every `,` and returns one cell per field, in order. Even an empty
  line has one field: a line with *n* commas always gives *n* + 1 cells.
- Each field is trimmed first: leading and trailing whitespace is removed, as `str::trim` does.
  An empty trimmed field becomes `Empty`. Otherwise, if `field.parse::<i64>()` returns `Ok(n)`, it
  becomes `Int(n)`, and anything else becomes a `Text` holding the trimmed field.
- `numeric_sum` returns the sum of the values of all `Int` cells, or `0` if there are none. The
  tests keep the sum within the range of `i64`.
- `texts` returns the strings of the `Text` cells in order, as slices borrowed from `row`.

### Examples

```rust
let row = parse_row("3, +5, 1.5,  blue sky , ");
// [Int(3), Int(5), Text("1.5"), Text("blue sky"), Empty]

numeric_sum(&row) // 8
texts(&row)       // ["1.5", "blue sky"]
parse_row("")     // [Empty]
```
