`grid[(x, y)]` is sugar for `*grid.index((x, y))`, or `*grid.index_mut((x, y))` when it is assigned
to or borrowed mutably. So `index` returns a **reference** to the cell (`&Self::Output`, here `&T`),
and `IndexMut` reuses the `Output` type of `Index`, which it requires. Because the methods hand out
references, indexing also works for cells that are not `Copy`, such as `g[(1, 0)].push_str("hi")`.

The bounds check must look at each coordinate. `self.cells[y * self.width + x]` only panics when the
flattened index is past the end of the `Vec`; `(3, 0)` on a 3 × 2 grid silently becomes `(0, 1)`.
Check `x < width && y < height` **before** computing the index: with a huge coordinate like
`usize::MAX`, `y * width + x` overflows, which panics in debug builds (and in Miri) instead of giving
`None`. The private helper `offset` does this once for `get`, `index` and `index_mut`.

### Alternatives

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

    // `row` stays as given.
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
