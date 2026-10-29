When a non-reference pattern such as a tuple, `Some(..)` or a struct pattern meets a reference, the
compiler skips the reference and switches the **default binding mode** from `move` to `ref`: every
binding below that point borrows. This is "match ergonomics". In `for (name, s) in scores`, each item
is a `&(String, u32)`, so `name` is a `&String` and `s` is a `&u32`.

In Rust 2021, `mut` quietly switched the mode back to `move`, so `(name, mut s)` made `s` a copied
`u32` while `name` stayed a `&String`. Rust 2024 forbids writing `ref`, `mut` or `&` where the
default binding mode is no longer `move`, and this `mut` is one of them.

Removing `mut` alone makes `s` a `&u32`, and `s += 5` no longer compiles; `*s += 5` would write
through a shared reference, which is not allowed either. Write the outer `&` yourself:
`&(ref name, mut s)` matches the reference with a reference pattern, so the pattern is **fully
explicit** (it skips no reference), the mode stays `move`, and `ref` and `mut` are allowed again.
Here `ref` is required, not redundant: `&(name, mut s)` would move the `String` out of the slice,
which does not compile. This is also what `cargo fix --edition` and the compiler suggest.

### Alternatives

Keep the short pattern and copy the score out in the body.

```rust
pub fn bonus_total(scores: &[(String, u32)]) -> u32 {
    let mut total = 0;
    for (name, s) in scores {
        let mut s = *s;
        if name.len() > 3 {
            s += 5;
        }
        total += s;
    }
    total
}
```

With iterator adapters, there is nothing to mutate. `s + 5` works on a `&u32` directly, but the other
branch needs `*s` so that both have the type `u32`.

```rust
pub fn bonus_total(scores: &[(String, u32)]) -> u32 {
    scores
        .iter()
        .map(|(name, s)| if name.len() > 3 { s + 5 } else { *s })
        .sum()
}
```
