Text stored in a field with a size limit often has to be cut short. Implement two helpers that
shorten a string to a number of bytes without ever splitting a character.

```rust
pub fn truncate_bytes(s: &str, max: usize) -> &str
pub fn ellipsize(s: &str, max: usize) -> String
```

A length in bytes is the length of the UTF-8 encoding, as returned by `str::len`.

- `truncate_bytes` returns the longest prefix of `s` that is at most `max` bytes long and does not
  split a character.
- `ellipsize` returns a copy of `s` if `s` is at most `max` bytes long. Otherwise it returns
  `truncate_bytes(s, max - 3)` followed by `'…'` (U+2026, which takes 3 bytes in UTF-8), so the
  result is never longer than `max` bytes. `max` is at least 3.
- `truncate_bytes` returns a part of `s` itself, not a new string or a string literal: a slice that
  starts where `s` starts. This holds for an empty result too. The tests check where the returned
  slice points.
- `max` can be any value, including 0 and values larger than the length of `s` (only `ellipsize`
  requires `max` to be at least 3).

### Examples

| Call | Result |
| --- | --- |
| `truncate_bytes("héllo", 2)` | `"h"` (`é` takes 2 bytes and does not fit) |
| `truncate_bytes("héllo", 3)` | `"hé"` |
| `ellipsize("こんにちは", 10)` | `"こん…"` (6 + 3 = 9 bytes) |
| `ellipsize("hello!", 5)` | `"he…"` |
| `ellipsize("hello", 5)` | `"hello"` |
