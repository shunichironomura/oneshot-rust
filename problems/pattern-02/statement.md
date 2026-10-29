The code below does not compile. Fix it.

The code was written for Rust 2021 and compiled there, but the judge compiles it as Rust 2024. It
totals a small scoreboard of `(name, score)` entries.

```rust
pub fn bonus_total(scores: &[(String, u32)]) -> u32;
```

- It returns the sum of the scores, where an entry whose name is longer than 3 bytes earns 5 extra
  points.
- The result always fits in a `u32`.

Do not change the signature.

### Examples

| Call | Result |
| --- | --- |
| `bonus_total(&[("anna".to_string(), 1), ("bo".to_string(), 2)])` | `8` (`1 + 5 + 2`) |
