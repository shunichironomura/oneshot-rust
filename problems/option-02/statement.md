A small web server limits how many times a user may fail to log in. Implement the function
`within_limit`.

```rust
pub fn within_limit(limit: Option<u32>, attempts: u32) -> bool
```

- `within_limit` tells whether a user who has failed to log in `attempts` times may try again.
- `None` means there is no limit, and the result is always `true`.
- `Some(n)` allows `n` attempts in total, so the result is `true` exactly when `attempts < n`.

### Examples

```rust
within_limit(None, 99)     // true
within_limit(Some(3), 2)   // true
within_limit(Some(3), 3)   // false
```
