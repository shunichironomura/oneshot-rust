`v.iter()` yields `&i64`, and `filter` passes a reference to each item, so its closure receives
`&&i64`. The pattern `|&&x|` strips both references. `sum()` knows to produce an `i64` from the
return type.

`x % 2 == 0` is also correct for negative numbers (`-4 % 2 == 0`). Testing for odd numbers with
`x % 2 == 1` would be wrong, because `-3 % 2 == -1`.

### Alternatives

`copied()` turns the items into plain `i64`, which makes the closures simpler.

```rust
pub fn sum_even_squares(v: &[i64]) -> i64 {
    v.iter().copied().filter(|x| x % 2 == 0).map(|x| x * x).sum()
}
```
