Implement `last_long`, which finds the last word that is long enough. It fits in a line, but every
detail of the rules below matters.

```rust
pub fn last_long(words: &[&str], min: usize) -> Option<usize>
```

- `last_long` returns the position of the **last** word that has at least `min` characters. The
  position is counted from the start of `words`: the first word is at position `0`.
- Characters are counted as `char`s (Unicode scalar values), not bytes: `"é"` and `"日"` are one
  character each.
- If no word is long enough (or `words` is empty), it returns `None`.

### Examples

- `last_long(&["tea", "coffee", "milk", "juice"], 5)` returns `Some(3)`.
- `last_long(&["abc", "日本"], 3)` returns `Some(0)`.
