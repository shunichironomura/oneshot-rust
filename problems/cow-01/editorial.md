All three parts follow the same rule: look first, and copy only when something really changes.

`Cow<'_, str>` is either `Borrowed(&str)` or `Owned(String)`, and the `'_` ties a borrowed result to
`s`. `s.trim_ascii()` is already a slice of `s`, so when nothing inside has to change it can be
returned as `Cow::Borrowed` without allocating anything. A `String` is built only when some run is
longer than one character or is not a plain space. Beware that `str::trim` and `split_whitespace`
use the Unicode definition of whitespace, which is wider than the ASCII one. Their ASCII versions
are `trim_ascii` and `split_ascii_whitespace`.

The owned form of `[i32]` is `Vec<i32>` (through `ToOwned`), so `Cow<'_, [i32]>` holds either a
`&[i32]` or a `Vec<i32>`. `to_mut` returns `&mut Vec<i32>`: on a `Borrowed` value it first copies the
slice into a new `Vec` and switches to `Owned`; on an `Owned` value it simply hands out the `Vec` it
already has. Calling it unconditionally would copy even when nothing exceeds `max`, so check with
`iter().any(...)` first. Calling `to_mut` inside a `for x in v.iter()` loop does not compile,
because the loop still borrows `v`.

`Rc` only gives shared access, so `self.lines[i] = ...` does not compile. `Rc::make_mut` is
clone-on-write for `Rc`: if other `Rc`s point to the same value, it clones the value into a new
allocation owned by this `Rc` alone; otherwise it returns `&mut` to the value in place. `Rc::get_mut`
is the strict version that returns `None` when the value is shared.

`==` on two `Rc`s compares the values, so two separately created `Doc`s with equal lines would look
shared. `Rc::ptr_eq` compares the pointers instead.

### Alternatives

Build the normalized string and compare it with the trimmed slice. This is shorter, but it allocates
even when the result is borrowed.

```rust
pub fn normalize_ws(s: &str) -> Cow<'_, str> {
    let trimmed = s.trim_ascii();
    let joined = trimmed.split_ascii_whitespace().collect::<Vec<_>>().join(" ");
    if joined == trimmed {
        Cow::Borrowed(trimmed)
    } else {
        Cow::Owned(joined)
    }
}
```

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

`set_line` can do by hand what `make_mut` does, with `Rc::strong_count` and `Rc::get_mut`. Be careful
to clone the `Vec`: `self.lines.clone()` would only clone the `Rc`.

```rust
pub fn set_line(&mut self, i: usize, text: &str) {
    if Rc::strong_count(&self.lines) > 1 {
        self.lines = Rc::new(Vec::clone(&self.lines));
    }
    Rc::get_mut(&mut self.lines).unwrap()[i] = text.to_string();
}
```
