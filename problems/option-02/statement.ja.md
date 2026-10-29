小さな Web サーバーが、ログインに失敗してよい回数を制限しています。関数 `within_limit` を実装してください。

```rust
pub fn within_limit(limit: Option<u32>, attempts: u32) -> bool
```

- `within_limit` は、ログインに `attempts` 回失敗したユーザーがもう一度試してよいかを返します。
- `None` は上限がないことを表し、結果は常に `true` です。
- `Some(n)` は合計 `n` 回まで試せることを表し、結果は `attempts < n` のときにだけ `true` です。

### 例

```rust
within_limit(None, 99)     // true
within_limit(Some(3), 2)   // true
within_limit(Some(3), 3)   // false
```
