A file name or a long ID shown in a narrow column is often cut down to its end. Implement the
function `last_chars`, which keeps only the last characters of a string.

```rust
pub fn last_chars(s: &str, n: usize) -> &str
```

A *character* is a Rust `char` (a Unicode scalar value), so `"e\u{301}"` (`e` followed by a
combining accent) counts as two characters.

- `last_chars` returns the last `n` characters of `s`, or all of `s` if it has `n` characters or
  fewer.
- It returns a part of `s` itself, not a new string or a string literal: a slice that ends where `s`
  ends. This holds for an empty result too. The tests check where the returned slice points.
- `n` can be any value, including 0 and values larger than the length of `s`.

### Examples

`last_chars("日本語です", 2)` is `"です"`.
