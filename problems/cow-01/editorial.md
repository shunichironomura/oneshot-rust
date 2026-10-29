The rule is: look first, and copy only when something really changes.

The owned form of `[i32]` is `Vec<i32>` (through `ToOwned`), so `Cow<'_, [i32]>` holds either a
`&[i32]` or a `Vec<i32>`. `to_mut` returns `&mut Vec<i32>`: on a `Borrowed` value it first copies the
slice into a new `Vec` and switches to `Owned`; on an `Owned` value it simply hands out the `Vec` it
already has. Calling it unconditionally would copy even when nothing exceeds `max`, so check with
`iter().any(...)` first. Calling `to_mut` inside a `for x in v.iter()` loop does not compile,
because the loop still borrows `v`.

### Alternatives

`clamp_all` can also match on the variant: change an `Owned` `Vec` in place, and replace a
`Borrowed` slice with a new clamped `Vec` only when needed. In the `Borrowed` arm, `s` borrows from
`v`, but its last use is on the right-hand side, so assigning to `*v` is accepted.

```rust
pub fn clamp_all(v: &mut Cow<'_, [i32]>, max: i32) {
    match v {
        Cow::Owned(vec) => {
            for x in vec.iter_mut() {
                *x = (*x).min(max);
            }
        }
        Cow::Borrowed(s) => {
            if s.iter().any(|&x| x > max) {
                *v = Cow::Owned(s.iter().map(|&x| x.min(max)).collect());
            }
        }
    }
}
```
