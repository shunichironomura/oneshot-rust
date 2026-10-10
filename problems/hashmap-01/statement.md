Count how many times each word appears in `text`.

```rust
pub fn word_count(text: &str) -> Vec<(String, usize)>
```

- Words are separated by whitespace and are **lowercased** before counting (`"Rust"` and `"rust"` are
  the same word).
- Sort the result by count, **highest first**. Words with the same count are sorted alphabetically.

### Example

`word_count("b a B c a b")` returns:

```text
[("b", 3), ("a", 2), ("c", 1)]
```
