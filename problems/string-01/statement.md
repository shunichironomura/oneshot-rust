Return a copy of `s` where the first character of every word is uppercased.

```rust
pub fn capitalize_words(s: &str) -> String
```

- A word is a run of non-whitespace characters. Whitespace (spaces, tabs, newlines) is kept exactly
  as it is.
- Only the first character of each word changes. The rest of the word is left unchanged, even if it
  is already uppercase.
- Uppercasing follows Unicode rules, so it works for any language. Some characters become more than
  one character (`'ß'` becomes `"SS"`). Characters without an uppercase form stay as they are.

### Examples

| Input | Output |
| --- | --- |
| `"hello world"` | `"Hello World"` |
| `"élan vital"` | `"Élan Vital"` |
