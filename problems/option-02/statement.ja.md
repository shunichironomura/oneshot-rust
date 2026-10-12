小さな Web サーバーが、ログイン中のユーザーを次の構造体 `Session` で管理しています。
これを扱う 4 つの補助関数を書いてください。`Session` はエディタに入っています。

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Session {
    pub user: String,
    pub expires_at: u64,
    pub admin: bool,
}

pub fn is_admin(s: Option<&Session>) -> bool
pub fn within_limit(limit: Option<u32>, attempts: u32) -> bool
pub fn time_left(s: &Session, now: u64) -> Option<u64>
pub fn evict_expired(slot: &mut Option<Session>, now: u64) -> Option<Session>
```

時刻の単位は秒です。セッションは `now < expires_at` の間は有効で、`expires_at <= now` になると
**期限切れ**です。`now` は `expires_at` より前でも後でもよく、`u64` のどの値も取りえます。

- `is_admin` は、セッションがあり、その `admin` フラグが立っていれば `true` を、そうでなければ
  `false` を返します。期限は見ません。
- `within_limit` は、ログインに `attempts` 回失敗したユーザーがもう一度試してよいかを返します。
  `None` は上限がないことを表し、結果は常に `true` です。`Some(n)` は合計 `n` 回まで試せることを表し、
  結果は `attempts < n` のときにだけ `true` です。
- `time_left` は、セッションの残り秒数 `Some(expires_at - now)` を返します。期限切れなら `None` です。
- `evict_expired` は、セッションが入っているかもしれないスロットを掃除します。スロットに期限切れの
  セッションが入っていれば、それをスロットから取り出して（スロットは `None` になります）返します。
  それ以外のとき（スロットが空か、セッションがまだ有効なとき）は `None` を返し、スロットは
  そのままにします。

### 例

```rust
let root = Session { user: "root".to_string(), expires_at: 100, admin: true };

is_admin(Some(&root))         // true
within_limit(Some(3), 3)      // false
time_left(&root, 40)          // Some(60)

let mut slot = Some(root.clone());
evict_expired(&mut slot, 100) // Some(root)。セッションが取り出され、slot は None になる
```
