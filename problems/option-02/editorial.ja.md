`Option::is_none_or(f)`（Rust 1.82）は、`None` なら `true`、`Some(x)` なら `f(x)` を返します。
`Some(x)` で `f(x)` が `true` のときにだけ `true` になる `is_some_and(f)` と対になるメソッドです。

`attempts < limit.unwrap_or(u32::MAX)` も同じに見えますが、`attempts == u32::MAX` のときに間違えます。
`u32::MAX` より大きい `u32` はないので、「上限なし」を `u32` の値で表すことはできないからです。

### 別解

普通の `match` でも同じように書けます。

```rust
pub fn within_limit(limit: Option<u32>, attempts: u32) -> bool {
    match limit {
        Some(max) => attempts < max,
        None => true,
    }
}
```

`is_none_or` ができる前は、`map_or` で書くのが一般的でした。

```rust
pub fn within_limit(limit: Option<u32>, attempts: u32) -> bool {
    limit.map_or(true, |max| attempts < max)
}
```
