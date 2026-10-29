The code below does not compile. Fix it.

The function `find_cell` searches a grid and stops as soon as it finds its answer.

```rust
pub fn find_cell(grid: &[Vec<i32>], target: i32) -> Option<(usize, usize)>
```

- `find_cell` returns the position `(row, column)` of the first cell whose value is `target`. The
  grid is scanned row by row from the top, and each row from left to right. Indices start at 0.
- Rows may have different lengths, and some rows may be empty.
- If no cell holds `target`, the result is `None`.

Do not change the signature.

### Examples

- `find_cell(&[vec![1, 2], vec![3, 2]], 2)` is `Some((0, 1))`, and
  `find_cell(&[vec![1, 2], vec![3, 2]], 4)` is `None`.
