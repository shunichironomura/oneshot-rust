Rust 2024 reserves `gen` as a keyword, for `gen` blocks (a planned way to write iterators); the
`rand` crate renamed its `Rng::gen` to `Rng::random` for this reason. In 2024 code the word `gen`
can no longer be an identifier, so both the definition `pub fn gen` and the call `self.gen()` in
`gen_below` are syntax errors. Renaming is not an option because the harness uses the name, so write
it as the raw identifier `r#gen` in both places. `r#gen` is the same name as `gen`: a 2021 crate
still calls this method as `.gen()`, while 2024 code such as the harness writes `rng.r#gen()`. Only
the exact word is reserved, so `gen_below` and the string `"gen"` need no change.

`type` has been a keyword in every edition, which is why the 2021 code already wrote the field as
`r#type`. A raw identifier works for local variables and in the field init shorthand
`Token { r#type, text: ... }` too, so `classify` compiles as it is.

The second change is quieter. Before 2024, calling `.into_iter()` on a `Box<[T]>` went through the
slice and yielded references `&T`, for backwards compatibility (Rust 2021 even warns about it with
the `boxed_slice_into_iter` lint). In 2024 the call uses `IntoIterator for Box<[T]>`, which consumes
the box and yields the values. So `p` is now a `u32`, and `*p` does not compile (E0614). Removing
the `*` is not enough: `into_iter()` now moves `points`, so the `points.len()` after the loop uses a
moved value (E0382). `points.iter()` keeps the old meaning, and it is also what
`cargo fix --edition` writes.

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
