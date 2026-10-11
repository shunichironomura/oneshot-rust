The code below does not compile. **Without looking at the error message**, predict where the compiler
complains and fix it.

Each of the three functions stops looking as soon as it knows its answer.

```rust
pub fn find_cell(grid: &[Vec<i32>], target: i32) -> Option<(usize, usize)>
pub fn parse_setting(line: &str) -> Option<(&str, u32)>
pub fn classify(xs: &[i32]) -> &'static str
```

- `find_cell` returns the position `(row, column)` of the first cell whose value is `target`. The
  grid is scanned row by row from the top, and each row from left to right. Indices start at 0.
  Rows may have different lengths, and some rows may be empty. If no cell holds `target`, the result
  is `None`.
- `parse_setting` reads a line of the form `key = value`. The line is split at its **first** `=`:
  the part before it is the key, and the part after it is the value. Both parts are trimmed with
  `str::trim`. The result is `Some((key, value))`, where `key` is the trimmed key (a slice of `line`)
  and `value` is the trimmed value parsed as a `u32`.
- `parse_setting` returns `None` if the line has no `=`, if the trimmed key is empty, or if
  `str::parse::<u32>` rejects the trimmed value (for example when it is empty, negative, not a
  number, or greater than `u32::MAX`).
- `classify` returns `"empty"` if `xs` is empty. Otherwise it returns `"negative"` if some element is
  less than 0, otherwise `"zero"` if some element is 0, and otherwise `"positive"`.

Do not change the signatures.

### Examples

- `find_cell(&[vec![1, 2], vec![3, 2]], 2)` is `Some((0, 1))`, and
  `find_cell(&[vec![1, 2], vec![3, 2]], 4)` is `None`.
- `parse_setting(" width = 80 ")` is `Some(("width", 80))`, and `parse_setting("height")` is
  `None`.
- `classify(&[3, 0, 5])` is `"zero"`, and `classify(&[4, -2])` is `"negative"`.
