`Option::is_none_or(f)` (Rust 1.82) is `true` for `None`, and `f(x)` for `Some(x)`. It is the
counterpart of `is_some_and(f)`, which is `true` only for a `Some(x)` where `f(x)` is `true`.

`attempts < limit.unwrap_or(u32::MAX)` looks equivalent, but it is wrong for
`attempts == u32::MAX`: no `u32` is larger than `u32::MAX`, so no `u32` limit can stand for
"no limit".

### Alternatives

A plain `match` works just as well.

```rust
pub fn within_limit(limit: Option<u32>, attempts: u32) -> bool {
    match limit {
        Some(max) => attempts < max,
        None => true,
    }
}
```

`map_or` was the usual way to write it before `is_none_or` existed.

```rust
pub fn within_limit(limit: Option<u32>, attempts: u32) -> bool {
    limit.map_or(true, |max| attempts < max)
}
```
