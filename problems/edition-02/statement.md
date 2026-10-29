The code below does not compile. Fix it.

The code was written for Rust 2021 and compiled there, but the judge compiles it as Rust 2024. It
computes a score from a boxed slice of points.

```rust
pub fn score(points: Box<[u32]>) -> u32;
```

- `score(points)` returns the sum of the entries plus 1 for each entry.
- An empty slice scores `0`.
- The result always fits in a `u32`.

Do not change the signature.

### Examples

- `score(vec![3, 4].into_boxed_slice())` is `3 + 4 + 2 = 9`.
