`start_bound()` and `end_bound()` return `Bound<&usize>`, a bound holding a **reference**. The
pattern `Bound::Included(&s)` copies the number out. An excluded start or an included end needs
`+ 1`, and `usize::MAX + 1` overflows: `checked_add(1)?` turns that into `None`, which is the right
answer because such a range reaches past every grid. Then `slice::get(start..end)` returns `None`
both when `end > len` and when `start > end`, which is exactly the rule of the statement.

`row(y)` panics for a row out of range, so check `y >= height` first and return `None`.

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
