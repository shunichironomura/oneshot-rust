The code below does not compile. Fix it.

The code was written for Rust 2021 and compiled there, but the judge compiles it as Rust 2024. Both
functions read the rounds of a small scoreboard.

```rust
pub fn rounds_over(scores: &[u32], par: u32) -> Vec<usize>;
pub fn initials(entries: &[(char, u32)]) -> String;
```

- `rounds_over` returns the positions (counted from 0) of the scores strictly greater than `par`, in
  increasing order. A score equal to `par` is not included.
- `initials` returns the characters of `entries`, in order, joined into one string.

Do not change the signatures.

### Examples

| Call | Result |
| --- | --- |
| `rounds_over(&[3, 7, 5, 9], 5)` | `[1, 3]` |
| `initials(&[('a', 1), ('b', 2)])` | `"ab"` |
