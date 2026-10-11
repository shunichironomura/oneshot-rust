`grid[(x, y)]` is sugar for `*grid.index((x, y))`, or `*grid.index_mut((x, y))` when it is assigned
to or borrowed mutably. So `index` returns a **reference** to the cell (`&Self::Output`, here `&T`),
and `IndexMut` reuses the `Output` type of `Index`, which it requires. Because the methods hand out
references, indexing also works for cells that are not `Copy`, such as `g[(1, 0)].push_str("hi")`.

The bounds check must look at each coordinate. `self.cells[y * self.width + x]` only panics when the
flattened index is past the end of the `Vec`; `(3, 0)` on a 3 × 2 grid silently becomes `(0, 1)`.
Check `x < width && y < height` **before** computing the index: with a huge coordinate like
`usize::MAX`, `y * width + x` overflows, which panics in debug builds (and in Miri) instead of giving
`None`. The private helper `offset` does this once for `get`, `index` and `index_mut`.

`row` needs its own check too. With `width == 0`, `&self.cells[y * 0..y * 0 + 0]` is an empty slice
for every `y`, so it would never panic.

For `row_sum`, `start_bound()` and `end_bound()` return `Bound<&usize>`, a bound holding a
**reference**. The pattern `Bound::Included(&s)` copies the number out. An excluded start or an
included end needs `+ 1`, and `usize::MAX + 1` overflows: `checked_add(1)?` turns that into `None`,
which is the right answer because such a range reaches past every grid. Then `slice::get(start..end)`
returns `None` both when `end > len` and when `start > end`, which is exactly the rule of the
statement.

Finally, sum in `i64`. `iter().sum::<i32>()` overflows for three cells of `i32::MAX`, so convert each
value with `i64::from` first. (`as i64` also works, but `from` makes it clear that nothing is lost.)

### Alternatives

Slices can be indexed by a pair of `Bound<usize>` directly, and `slice::get` with such a pair does
the overflow checks itself. `Bound::cloned` turns `Bound<&usize>` into `Bound<usize>`.

```rust
impl Grid<i32> {
    pub fn row_sum<R: RangeBounds<usize>>(&self, y: usize, cols: R) -> Option<i64> {
        if y >= self.height {
            return None;
        }
        let range = (cols.start_bound().cloned(), cols.end_bound().cloned());
        let cells = self.row(y).get(range)?;
        Some(cells.iter().map(|&v| i64::from(v)).sum())
    }
}
```

The indexing can also be built on `get` and a private `get_mut`, with `expect` producing the panic.

```rust
impl<T> Grid<T> {
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            self.cells.get(y * self.width + x)
        } else {
            None
        }
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x < self.width && y < self.height {
            self.cells.get_mut(y * self.width + x)
        } else {
            None
        }
    }

    // `row` stays as in the model solution.
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &T {
        self.get(x, y).expect("cell out of range")
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut T {
        self.get_mut(x, y).expect("cell out of range")
    }
}
```
