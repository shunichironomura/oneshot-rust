Copying is expensive, so copy only when you actually have to change something. Implement
`normalize_ws`, which cleans up the whitespace in `s`.

```rust
use std::borrow::Cow;

pub fn normalize_ws(s: &str) -> Cow<'_, str>;
```

- Whitespace means the characters for which `char::is_ascii_whitespace` is true.
- The result is `s` with the whitespace at both ends removed and every run of whitespace between
  two non-whitespace characters replaced by a single `' '`.
- If no run has to be replaced (every run inside is already exactly one `' '`), the result is a part
  of `s` as it is: return it as `Cow::Borrowed`, a slice of `s`, without copying. Otherwise return
  `Cow::Owned`.

### Examples

```rust
normalize_ws("a b")       // Borrowed("a b")
normalize_ws("  hi\n")    // Borrowed("hi"), a slice of the input
normalize_ws("a  b\tc ")  // Owned("a b c")
```
