The rule is: look first, and copy only when something really changes.

`Cow<'_, str>` is either `Borrowed(&str)` or `Owned(String)`, and the `'_` ties a borrowed result to
`s`. `s.trim_ascii()` is already a slice of `s`, so when nothing inside has to change it can be
returned as `Cow::Borrowed` without allocating anything. A `String` is built only when some run is
longer than one character or is not a plain space. Beware that `str::trim` and `split_whitespace`
use the Unicode definition of whitespace, which is wider than the ASCII one. Their ASCII versions
are `trim_ascii` and `split_ascii_whitespace`.

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
