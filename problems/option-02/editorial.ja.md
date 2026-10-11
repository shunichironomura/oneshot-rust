`Option` と `bool` の補助メソッドを使えば、どの関数も 1〜3 行で書けます。ただし、どのメソッドにも、
記憶を頼りに書くと間違えやすい点があります。

- `is_admin`：`Option::is_some_and(f)` は、`Some(x)` で `f(x)` が `true` のときにだけ `true` です。
- `within_limit`：`Option::is_none_or(f)`（Rust 1.82）はその対になるメソッドで、`None` なら `true`、
  `Some(x)` なら `f(x)` を返します。`attempts < limit.unwrap_or(u32::MAX)` も同じに見えますが、
  `attempts == u32::MAX` のときに間違えます。`u32::MAX` より大きい `u32` はないので、「上限なし」を
  `u32` の値で表すことはできないからです。
- `time_left`：`bool::then_some(v)` に渡す値は、ほかの関数の引数と同じく、条件が真でも偽でも
  呼び出しの**前に**計算されます。そのため `(now < s.expires_at).then_some(s.expires_at - now)` は
  期限切れのセッションでも引き算をしてしまい、`now > expires_at` になるとオーバーフローします。
  デバッグビルドや Miri ではパニックになります。`bool::then` はクロージャを受け取り、条件が真の
  ときにだけ実行します。`checked_sub` を使えばオーバーフローは避けられますが、それだけでは
  `now == expires_at` のときに `Some(0)` を返してしまいます。
- `evict_expired`：`Option::take_if(f)`（Rust 1.80）は `&mut self` を受け取ります。`f` に
  `&mut T` を渡して呼び、`f` が `true` を返したときにだけ値を取り出して `None` を残します。
  ほかの思いつきやすい書き方はうまくいきません。
  - `slot.filter(...)`、`slot.is_some_and(...)`、`slot.map(...)`、`slot.unwrap()` は `Option` を
    **値で**受け取ります。`slot: &mut Option<Session>` を通して呼ぶと、参照の先からのムーブになります
    （E0507）。`is_admin` と `within_limit` で同じメソッドを使えたのは、これらの関数が `Option` を
    値で受け取っていて、消費してよいからです。
  - `slot.take().filter(...)` はコンパイルが通りますが、`take` がまずスロットを必ず空にするので、
    まだ有効なセッションは戻されずに捨てられます。
  - `slot.as_ref().filter(...).cloned()` はコピーを返すだけで、期限切れのセッションはスロットに残ります。
- `authorize`：`Option::ok_or(e)` は `None` を `Err(e)` に変え、`bool::ok_or(e)`（Rust 1.98）は `false` を
  `Err(e)` に、`true` を `Ok(())` に変えます。`?` と組み合わせると検査 1 つが 1 行になり、行の順番が
  そのまま検査の順番になります。最後の検査には `?` は要りません。その `Result<(), AuthError>` が
  そのまま返り値になるからです。

### 別解

普通の `match` と `if` でも同じように書けます。`evict_expired` の `session` は `slot` を借用していますが、
使っているのはガードだけなので、`slot.take()` を実行するときには借用は終わっています。

```rust
pub fn is_admin(s: Option<&Session>) -> bool {
    match s {
        Some(session) => session.admin,
        None => false,
    }
}

pub fn within_limit(limit: Option<u32>, attempts: u32) -> bool {
    match limit {
        Some(max) => attempts < max,
        None => true,
    }
}

pub fn time_left(s: &Session, now: u64) -> Option<u64> {
    if now < s.expires_at { Some(s.expires_at - now) } else { None }
}

pub fn evict_expired(slot: &mut Option<Session>, now: u64) -> Option<Session> {
    match slot {
        Some(session) if session.expires_at <= now => slot.take(),
        _ => None,
    }
}

pub fn authorize(s: Option<&Session>, now: u64) -> Result<(), AuthError> {
    let Some(s) = s else {
        return Err(AuthError::NoSession);
    };
    if s.expires_at <= now {
        Err(AuthError::Expired)
    } else if !s.admin {
        Err(AuthError::NotAdmin)
    } else {
        Ok(())
    }
}
```

`time_left` は、0 になった結果を取り除けば `checked_sub` でも書けます。`is_none_or` ができる前は、
`within_limit` を `map_or` で書くのが一般的でした。

```rust
pub fn time_left(s: &Session, now: u64) -> Option<u64> {
    s.expires_at.checked_sub(now).filter(|&left| left > 0)
}

pub fn within_limit(limit: Option<u32>, attempts: u32) -> bool {
    limit.map_or(true, |max| attempts < max)
}
```

`as_ref()` を通して見れば、スロットにも `is_some_and` を使えます。`as_ref()` は何もムーブせずに
`Option<&Session>` を作ります。

```rust
pub fn evict_expired(slot: &mut Option<Session>, now: u64) -> Option<Session> {
    if slot.as_ref().is_some_and(|session| session.expires_at <= now) {
        slot.take()
    } else {
        None
    }
}
```
