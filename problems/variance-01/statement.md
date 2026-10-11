The code below does not compile. **Without looking at the error message**, predict where the compiler
complains and fix it.

Two small text utilities:

```rust
pub fn merge(defaults: &[&'static str], extra: &str) -> Vec<String>
pub fn unique_upper(text: &str) -> Vec<String>
```

`merge` combines a list of built-in options with options that the user gives as one comma-separated
string.

- The result starts with every element of `defaults`, in order. `defaults` contains no duplicates.
- `extra` is split at every `,`. Each part is trimmed (whitespace at both ends is removed), and parts
  that are then empty are skipped.
- The remaining parts are appended in order, except that a part equal to a string already in the
  result (a default or an earlier part) is skipped. The comparison is exact and case-sensitive.

`unique_upper` lists the distinct words of a text in uppercase.

- `text` is split into words at whitespace. A run of whitespace characters (spaces, tabs, newlines,
  ...) counts as one separator, as with `split_whitespace`.
- Only the first occurrence of each word is kept, and the words are returned in order of first
  appearance. Words are compared exactly as they appear in `text`, before uppercasing, so `"a A"`
  gives `["A", "A"]`.
- Each kept word is uppercased with `to_uppercase`, which follows Unicode (`"straße"` becomes
  `"STRASSE"`).

Do not change the signatures of `merge` and `unique_upper`. The private helpers may be changed or
removed.

### Examples

- `merge(&["a", "b"], "b, c,,d")` returns `["a", "b", "c", "d"]`.
- `unique_upper("to be or not to be")` returns `["TO", "BE", "OR", "NOT"]`.
