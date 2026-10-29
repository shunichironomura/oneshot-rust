`Option::take_if(f)`（Rust 1.80）は `&mut self` を受け取ります。`f` に `&mut T` を渡して呼び、
`f` が `true` を返したときにだけ値を取り出して `None` を残します。ほかの思いつきやすい書き方は
うまくいきません。

- `slot.filter(...)`、`slot.is_some_and(...)`、`slot.map(...)`、`slot.unwrap()` は `Option` を
  **値で**受け取ります。`slot: &mut Option<Session>` を通して呼ぶと、参照の先からのムーブになります
  （E0507）。これらは、関数が所有していて消費してよい `Option` に使うメソッドです。
- `slot.take().filter(...)` はコンパイルが通りますが、`take` がまずスロットを必ず空にするので、
  まだ有効なセッションは戻されずに捨てられます。
- `slot.as_ref().filter(...).cloned()` はコピーを返すだけで、期限切れのセッションはスロットに残ります。

### 別解

普通の `match` でも同じように書けます。`session` は `slot` を借用していますが、使っているのはガードだけ
なので、`slot.take()` を実行するときには借用は終わっています。

```rust
pub fn evict_expired(slot: &mut Option<Session>, now: u64) -> Option<Session> {
    match slot {
        Some(session) if session.expires_at <= now => slot.take(),
        _ => None,
    }
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
