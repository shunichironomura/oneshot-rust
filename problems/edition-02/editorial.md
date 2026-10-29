Before 2024, calling `.into_iter()` on a `Box<[T]>` only borrowed the box and yielded references
`&T`, for backwards compatibility (Rust 2021 even warns about it with the `boxed_slice_into_iter`
lint). In 2024 the call uses `IntoIterator for Box<[T]>`, which consumes the box and yields the
values.

- So `p` is now a `u32`, and `*p` does not compile (E0614).
- Removing the `*` is not enough: `into_iter()` now moves `points`, so the `points.len()` after the
  loop uses a moved value (E0382).

`points.iter()` keeps the old meaning, and it is also what `cargo fix --edition` writes.

### Alternatives

Read the length first, then consume the box. `for p in points` iterates over the values.

```rust
pub fn score(points: Box<[u32]>) -> u32 {
    let bonus = points.len() as u32;
    let mut total = 0;
    for p in points {
        total += p;
    }
    total + bonus
}
```

With an iterator, each entry adds its points plus 1.

```rust
pub fn score(points: Box<[u32]>) -> u32 {
    points.iter().map(|p| p + 1).sum()
}
```
