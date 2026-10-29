小さな Web サーバーが、ログイン中のユーザーを次の構造体 `Session` で管理しています。セッション表の
各スロットには、セッションが 1 つ入っているかもしれません。関数 `evict_expired` を実装してください。
`Session` はエディタに入っています。

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Session {
    pub user: String,
    pub expires_at: u64,
    pub admin: bool,
}

pub fn evict_expired(slot: &mut Option<Session>, now: u64) -> Option<Session>
```

時刻の単位は秒です。セッションは `now < expires_at` の間は有効で、`expires_at <= now` になると
**期限切れ**です。`now` は `expires_at` より前でも後でもよく、`u64` のどの値も取りえます。

- スロットに期限切れのセッションが入っていれば、`evict_expired` はそれをスロットから取り出して
  （スロットは `None` になります）返します。
- それ以外のとき（スロットが空か、セッションがまだ有効なとき）は `None` を返し、スロットは
  そのままにします。

### 例

```rust
let root = Session { user: "root".to_string(), expires_at: 100, admin: true };
let mut slot = Some(root.clone());
evict_expired(&mut slot, 100) // Some(root)。セッションが取り出され、slot は None になる
```
