When a non-reference pattern such as a tuple meets a reference, the compiler skips the reference and
switches the **default binding mode** from `move` to `ref`: every binding below that point borrows.
This is "match ergonomics". Rust 2024 forbids writing `ref`, `mut` or `&` below a reference that the
pattern skipped this way.

`filter` passes each item **by reference**, so its closure receives an `&(usize, &u32)`, one more
reference than the items have. The pattern `(_, &s)` skips that outer reference, so its `&` is now an
error. `|&(_, &s)|` spells out both references, and `s` is a `u32`. Dropping the `&` instead makes
`s` a `&&u32`, and neither `s > par` nor `*s > par` compiles (`**s > par` does).

`initials` was correct all along: `|&(c, _)|` starts with `&`, so the pattern is **fully explicit**
(it skips no reference) and `c` is a `char`. The rule is not "no `&` in closure parameters" but "no
`ref`, `mut` or `&` below a reference the pattern skipped". Likewise `.map(|(round, _)| round)`
receives an `(usize, &u32)` by value, so nothing is skipped there.

`cargo fix --edition` makes the same change as the model solution: it makes the pattern fully
explicit.

### Alternatives

Keep the short pattern and dereference in the body.

```rust
pub fn rounds_over(scores: &[u32], par: u32) -> Vec<usize> {
    scores
        .iter()
        .enumerate()
        .filter(|(_, s)| **s > par)
        .map(|(round, _)| round)
        .collect()
}
```

`filter_map` receives each item by value, so `(round, &s)` skips no reference and the `&` is
allowed there.

```rust
pub fn rounds_over(scores: &[u32], par: u32) -> Vec<usize> {
    scores
        .iter()
        .enumerate()
        .filter_map(|(round, &s)| (s > par).then_some(round))
        .collect()
}
```
