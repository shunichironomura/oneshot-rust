A `Grid<T>` stores a `width` × `height` rectangle of cells in one `Vec<T>`, row by row (it is
already in the editor). Implement its constructor, `get`, and indexing with `grid[(x, y)]`.

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
    pub fn row(&self, y: usize) -> &[T]; // already implemented
}

impl<T> Index<(usize, usize)> for Grid<T> { type Output = T; /* ... */ }
impl<T> IndexMut<(usize, usize)> for Grid<T> { /* ... */ }
```

- A cell is addressed by `(x, y)`: column `x` in `0..width` and row `y` in `0..height`. Cell
  `(x, y)` is stored at index `y * width + x` of `cells`.
- `new` creates a grid whose cells are all `fill`. `width` and `height` may be `0`.
- `get(x, y)` returns the cell, or `None` if `x >= width` or `y >= height`. This holds for any `x`
  and `y`, including `usize::MAX`.
- `grid[(x, y)]` reads the cell, and `grid[(x, y)] = v` (or `+=` and so on) changes it. Both panic
  whenever `get(x, y)` would return `None`. Each coordinate is checked on its own: on a 3 × 2 grid,
  `(3, 0)` panics even though `0 * 3 + 3` is a valid index of `cells`.
- `row(y)` is given: it returns row `y` as a slice of `width` cells, ordered by `x`.

### Examples

```rust
let mut g = Grid::new(3, 2, 0);
g[(2, 1)] = 5;
g[(0, 0)] += 7;
g[(0, 0)]           // 7
g.get(2, 1)         // Some(&5)
g.get(3, 0)         // None
g.row(1)            // [0, 0, 5]
```
