The code below does not compile. Fix it.

It contains a small number utility.

```rust
pub fn digital_root(n: u64) -> u64
```

- `digital_root(n)` repeatedly replaces `n` by the sum of its decimal digits until it is less than
  10, and returns that number. A number that is already less than 10 (including `0`) is returned as
  it is.

Do not change the signature.

### Examples

- `digital_root(9875)` is `2` (9875 → 29 → 11 → 2).
