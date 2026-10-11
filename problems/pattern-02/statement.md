The code below does not compile. **Without looking at the error message**, predict where the compiler
complains and fix it.

The code was written for Rust 2021 and compiled there, but the judge compiles it as Rust 2024. It
keeps a small scoreboard. `Player` is already in the editor.

```rust
pub struct Player {
    pub name: String,
    pub score: u32,
}

pub fn name_len(nick: &Option<String>) -> usize;
pub fn leader(players: &[Player]) -> Option<&str>;
pub fn bonus_total(scores: &[(String, u32)]) -> u32;
pub fn rounds_over(scores: &[u32], par: u32) -> Vec<usize>;
pub fn initials(entries: &[(char, u32)]) -> String;
```

- `name_len` returns the length of the nickname in bytes (as `String::len` counts it), or `0` for
  `None`.
- `leader` returns the name of the player with the highest score. On a tie, the first of the tied
  players wins. It returns `None` if `players` is empty.
- `bonus_total` returns the sum of the scores, where an entry whose name is longer than 3 bytes earns
  5 extra points. The result always fits in a `u32`.
- `rounds_over` returns the positions (counted from 0) of the scores strictly greater than `par`, in
  increasing order. A score equal to `par` is not included.
- `initials` returns the characters of `entries`, in order, joined into one string.

Do not change the signatures.

### Examples

| Call | Result |
| --- | --- |
| `name_len(&Some("anna".to_string()))` | `4` |
| `leader(&players)` for anna (3 points), bo (9) and cy (4) | `Some("bo")` |
| `bonus_total(&[("anna".to_string(), 1), ("bo".to_string(), 2)])` | `8` (`1 + 5 + 2`) |
| `rounds_over(&[3, 7, 5, 9], 5)` | `[1, 3]` |
| `initials(&[('a', 1), ('b', 2)])` | `"ab"` |
