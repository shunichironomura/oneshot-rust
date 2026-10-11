Text shown in a narrow column, or stored in a field with a size limit, often has to be cut short.
Implement four helpers that shorten a string without ever splitting a character.

```rust
pub fn first_chars(s: &str, n: usize) -> &str
pub fn last_chars(s: &str, n: usize) -> &str
pub fn truncate_bytes(s: &str, max: usize) -> &str
pub fn ellipsize(s: &str, max: usize) -> String
```

A *character* is a Rust `char` (a Unicode scalar value), so `"e\u{301}"` (`e` followed by a
combining accent) counts as two characters. A length in bytes is the length of the UTF-8 encoding,
as returned by `str::len`.

- `first_chars` returns the first `n` characters of `s`, or all of `s` if it has `n` characters or
  fewer.
- `last_chars` returns the last `n` characters of `s`, or all of `s` if it has `n` characters or
  fewer.
- `truncate_bytes` returns the longest prefix of `s` that is at most `max` bytes long and does not
  split a character.
- `ellipsize` returns a copy of `s` if `s` is at most `max` bytes long. Otherwise it returns
  `truncate_bytes(s, max - 3)` followed by `'…'` (U+2026, which takes 3 bytes in UTF-8), so the
  result is never longer than `max` bytes. `max` is at least 3.
- The three functions that return `&str` return a part of `s` itself, not a new string or a string
  literal: `first_chars` and `truncate_bytes` return a slice that starts where `s` starts, and
  `last_chars` returns a slice that ends where `s` ends. This holds for empty results too. The tests
  check where the returned slices point.
- `n` and `max` can be any value, including 0 and values larger than the length of `s` (only
  `ellipsize` requires `max` to be at least 3).

### Examples

| Call | Result |
| --- | --- |
| `first_chars("héllo", 2)` | `"hé"` |
| `last_chars("日本語です", 2)` | `"です"` |
| `truncate_bytes("héllo", 2)` | `"h"` (`é` takes 2 bytes and does not fit) |
| `truncate_bytes("héllo", 3)` | `"hé"` |
| `ellipsize("こんにちは", 10)` | `"こん…"` (6 + 3 = 9 bytes) |
| `ellipsize("hello!", 5)` | `"he…"` |
| `ellipsize("hello", 5)` | `"hello"` |
