A `Grid<T>` stores a `width` × `height` rectangle of cells in one `Vec<T>`, row by row. The grid is
already in the editor, with `new`, `get`, `row` and indexing with `grid[(x, y)]`. Implement a row
sum for grids of `i32` that accepts any kind of range.

```rust
impl Grid<i32> {
    pub fn row_sum<R: RangeBounds<usize>>(&self, y: usize, cols: R) -> Option<i64>;
}
```

- Cell `(x, y)` is in column `x` (`0..width`) and row `y` (`0..height`). `row(y)` returns row `y`
  as a slice of `width` cells, ordered by `x`, and panics if `y >= height`.
- `row_sum(y, cols)` adds up the cells of row `y` in the columns selected by `cols`. `cols` can be
  any range: `..`, `a..`, `..b`, `..=b`, `a..b`, `a..=b`, or a pair of `std::ops::Bound` values such
  as `(Bound::Excluded(a), Bound::Included(b))`. It selects the columns `start..end`, where:
  - `start` is `a` for an included start, `a + 1` for an excluded start, and `0` if unbounded;
  - `end` is `b + 1` for an included end, `b` for an excluded end, and `width` if unbounded.
- `row_sum` returns `None` if `y >= height`, `end > width`, or `start > end`. These are exact
  comparisons: `..=usize::MAX`, for example, ends after column `usize::MAX`, so it gives `None` on
  every grid.
- Otherwise `row_sum` returns the exact sum of the selected cells, which may not fit in an `i32`.
  If `start == end`, nothing is selected and the result is `Some(0)`. On a grid of width 3, `3..3`
  gives `Some(0)`, but `4..4` gives `None` because `end > width`.

### Examples

```rust
let mut g = Grid::new(3, 2, 0);
g[(2, 1)] = 5;
g.row_sum(1, ..)    // Some(5)
g.row_sum(1, ..2)   // Some(0)
```

With a 4 × 1 grid whose row is `[10, 20, 30, 40]`:

```rust
g.row_sum(0, 1..3)                                         // Some(50)
g.row_sum(0, ..=2)                                         // Some(60)
g.row_sum(0, 2..)                                          // Some(70)
g.row_sum(0, (Bound::Excluded(0), Bound::Included(2)))     // Some(50)
g.row_sum(0, 3..5)                                         // None (past the width)
```
