Split `s` on whitespace and return the longest word. If several words share the longest length,
return the first one. If there are no words, return the empty string.

```rust
pub fn longest_word(s: &str) -> &str
```

The result must be a substring of `s` (a slice pointing into `s`).

### Examples

| Input | Output |
| --- | --- |
| `"a bb ccc"` | `"ccc"` |
| `"xx yy"` | `"xx"` |
