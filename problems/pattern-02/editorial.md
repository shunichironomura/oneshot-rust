When a non-reference pattern such as `Some(..)`, a tuple or a struct pattern meets a reference, the
compiler skips the reference and switches the **default binding mode** from `move` to `ref`: every
binding below that point borrows. This is "match ergonomics". Matching `Some(s)` against a
`&Option<String>` makes `s` a `&String`, and in `for Player { name, score } in players` both bindings
are references, which is why the code writes `*score`.

Rust 2024 forbids writing `ref`, `mut` or `&` where the default binding mode is no longer `move`. Each
broken function trips over one of them.

- `ref` (`name_len`, `leader`): it is redundant there, since the binding borrows anyway, and it is now
  an error. Simply remove it.
- `mut` (`bonus_total`): in 2021, `mut` quietly switched the mode back to `move`, so `(name, mut s)`
  made `s` a copied `u32` while `name` stayed a `&String`. Removing `mut` alone makes `s` a `&u32`,
  and `s += 5` no longer compiles; `*s += 5` would write through a shared reference, which is not
  allowed either. Write the outer `&` yourself: `&(ref name, mut s)` matches the reference with a
  reference pattern, so the mode stays `move` and `ref` and `mut` are allowed again. Here `ref` is
  required, not redundant: `&(name, mut s)` would move the `String` out of the slice, which does not
  compile. Copying the value out with `let mut s = *s;` works too.
- `&` (`rounds_over`): `filter` passes each item **by reference**, so its closure receives an
  `&(usize, &u32)`. The pattern `(_, &s)` skips that outer reference, and a `&` below a skipped
  reference is now an error. `|&(_, &s)|` spells out both references. Dropping the `&` instead
  makes `s` a `&&u32`, and `s > par` does not compile (`**s > par` does).

`initials` was correct all along: `|&(c, _)|` starts with `&`, so the pattern is **fully explicit**
(it skips no reference) and `c` is a `char`. The rule is not "no `&` in closure parameters" but "no
`ref`, `mut` or `&` below a reference the pattern skipped". Likewise `.map(|(round, _)| round)`
receives an `(usize, &u32)` by value, so nothing is skipped there.

`cargo fix --edition` makes the same changes as the compiler's suggestions, and so does the model
solution: it removes a redundant `ref`, and makes the pattern fully explicit where `mut` or `&` needs
the `move` mode.

### Alternatives

Keep the short patterns and copy or dereference in the body.

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

pub fn rounds_over(scores: &[u32], par: u32) -> Vec<usize> {
    scores
        .iter()
        .enumerate()
        .filter(|(_, s)| **s > par)
        .map(|(round, _)| round)
        .collect()
}
```

A fully explicit pattern fixes the `ref` cases too. Note that the types of the bindings change: with
`&Player { ref name, score }`, `score` is a copied `u32`, so the `*` has to go.

```rust
pub fn name_len(nick: &Option<String>) -> usize {
    match nick {
        &Some(ref s) => s.len(),
        &None => 0,
    }
}

pub fn leader(players: &[Player]) -> Option<&str> {
    let mut best: Option<(&str, u32)> = None;
    for &Player { ref name, score } in players {
        let better = match best {
            Some((_, top)) => score > top,
            None => true,
        };
        if better {
            best = Some((name, score));
        }
    }
    best.map(|(name, _)| name)
}
```

With iterator adapters. `filter_map` receives each item by value, so `(round, &s)` skips no
reference and the `&` is allowed there.

```rust
pub fn bonus_total(scores: &[(String, u32)]) -> u32 {
    scores
        .iter()
        .map(|(name, s)| if name.len() > 3 { s + 5 } else { *s })
        .sum()
}

pub fn rounds_over(scores: &[u32], par: u32) -> Vec<usize> {
    scores
        .iter()
        .enumerate()
        .filter_map(|(round, &s)| (s > par).then_some(round))
        .collect()
}
```
