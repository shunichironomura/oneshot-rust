The code below does not compile. Fix it.

`merge` combines a list of built-in options with options that the user gives as one comma-separated
string.

```rust
pub fn merge(defaults: &[&'static str], extra: &str) -> Vec<String>
```

- The result starts with every element of `defaults`, in order. `defaults` contains no duplicates.
- `extra` is split at every `,`. Each part is trimmed (whitespace at both ends is removed), and parts
  that are then empty are skipped.
- The remaining parts are appended in order, except that a part equal to a string already in the
  result (a default or an earlier part) is skipped. The comparison is exact and case-sensitive.

Do not change the signature of `merge`. The private helper may be changed or removed.

### Examples

- `merge(&["a", "b"], "b, c,,d")` returns `["a", "b", "c", "d"]`.
