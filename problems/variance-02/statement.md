The code below does not compile. Fix it.

`unique_upper` lists the distinct words of a text in uppercase.

```rust
pub fn unique_upper(text: &str) -> Vec<String>
```

- `text` is split into words at whitespace. A run of whitespace characters (spaces, tabs, newlines,
  ...) counts as one separator, as with `split_whitespace`.
- Only the first occurrence of each word is kept, and the words are returned in order of first
  appearance. Words are compared exactly as they appear in `text`, before uppercasing, so `"a A"`
  gives `["A", "A"]`.
- Each kept word is uppercased with `to_uppercase`, which follows Unicode (`"straße"` becomes
  `"STRASSE"`).

Do not change the signature of `unique_upper`. The private helper may be changed or removed.

### Examples

- `unique_upper("to be or not to be")` returns `["TO", "BE", "OR", "NOT"]`.
