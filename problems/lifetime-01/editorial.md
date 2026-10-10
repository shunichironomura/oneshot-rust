With exactly one reference parameter, lifetime elision ties the returned `&str` to `s`, so no
annotation is needed. `split_whitespace` yields slices of `s`, which can be returned as they are.

`fold` replaces the current best only when a word is **strictly** longer (`>`), so the first of
several equally long words wins.

### Alternatives

`max_by_key(|w| w.len())` looks natural, but it returns the **last** of several equal maxima. Scan
from the back with `rev()` to get the first one.

```rust
pub fn longest_word(s: &str) -> &str {
    s.split_whitespace().rev().max_by_key(|w| w.len()).unwrap_or("")
}
```
