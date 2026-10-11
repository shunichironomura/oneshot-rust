A `Grid<T>` stores a `width` × `height` rectangle of cells in one `Vec<T>`, row by row (it is
already in the editor). Implement its constructor and accessors, indexing with `grid[(x, y)]`, and
a row sum for grids of `i32`.

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    cells: Vec<T>,
}

impl<T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize, fill: T) -> Self;
}

impl<T> Grid<T> {
    pub fn get(&self, x: usize, y: usize) -> Option<&T>;
    pub fn row(&self, y: usize) -> &[T];
}

impl<T> Index<(usize, usize)> for Grid<T> { type Output = T; /* ... */ }
impl<T> IndexMut<(usize, usize)> for Grid<T> { /* ... */ }

impl Grid<i32> {
    pub fn row_sum<R: RangeBounds<usize>>(&self, y: usize, cols: R) -> Option<i64>;
}
```

- A cell is addressed by `(x, y)`: column `x` in `0..width` and row `y` in `0..height`. Cell
  `(x, y)` is stored at index `y * width + x` of `cells`.
- `new` creates a grid whose cells are all `fill`. `width` and `height` may be `0`.
- `get(x, y)` returns the cell, or `None` if `x >= width` or `y >= height`. This holds for any `x`
  and `y`, including `usize::MAX`.
- `grid[(x, y)]` reads the cell, and `grid[(x, y)] = v` (or `+=` and so on) changes it. Both panic
  whenever `get(x, y)` would return `None`. Each coordinate is checked on its own: on a 3 × 2 grid,
  `(3, 0)` panics even though `0 * 3 + 3` is a valid index of `cells`.
- `row(y)` returns row `y` as a slice of `width` cells, ordered by `x`. It panics if `y >= height`,
  even when `width` is `0`.
- `row_sum(y, cols)` adds up the cells of row `y` in the columns selected by `cols`. `cols` can be
  any range: `..`, `a..`, `..b`, `..=b`, `a..b`, `a..=b`, or a pair of `std::ops::Bound` values such
  as `(Bound::Excluded(a), Bound::Included(b))`. It selects the columns `start..end`, where:
  - `start` is `a` for an included start, `a + 1` for an excluded start, and `0` if unbounded;
  - `end` is `b + 1` for an included end, `b` for an excluded end, and `width` if unbounded.
- `row_sum` returns `None` if `y >= height`, `end > width`, or `start > end`. These are exact
  comparisons: `..=usize::MAX`, for example, ends after column `usize::MAX`, so it gives `None` on
  every grid. Otherwise `row_sum` returns the exact sum of the selected cells, which may not fit in
  an `i32`; if `start == end`, nothing is selected and the result is `Some(0)`. On a grid of width
  3, for example, `3..3` gives `Some(0)`, but `4..4` gives `None` because `end > width`.

### Examples

```rust
let mut g = Grid::new(3, 2, 0);
g[(2, 1)] = 5;
g[(0, 0)] += 7;
g[(0, 0)]           // 7
g.get(2, 1)         // Some(&5)
g.row(1)            // [0, 0, 5]
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
