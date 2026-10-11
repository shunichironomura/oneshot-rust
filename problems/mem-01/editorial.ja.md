値で束縛するパターン（`queue`、`session`、`other`）を使って `match self.state` と書くと、
中身が `self.state` からムーブされます。しかし `self` は `&mut Client` にすぎません。
クライアントの持ち主は呼び出し側で、Rust は参照の先にある場所が空になることを許しません。
直後に `self.state = ...` で新しい値を戻すとしても、その間だけ空にすることもできません
（途中でパニックが起きると、呼び出し側には状態がムーブされたあとのクライアントが残ってしまうからです）。
これが E0507 で、`take_closed` の `let closed = self.closed;` も同じ理由でエラーになります。

コンパイラの提案はここでは役に立ちません。`ref queue` や `&self.closed` は借用するだけですが、
次の状態はキューを**所有**しなければならず（`sent` や `unsent` になります）、
閉じたセッションの一覧も `Session` を所有しなければなりません。「`Clone` を実装すればよい」という注記も
役に立ちません。型にトレイトを追加することは禁じられていますし、セッションは複製するものではなく、
ムーブするものです。

`std::mem::take(&mut self.state)` は、その場所にデフォルト値を入れ、元の状態を値として取り出すことを
1 回で行います。`Conn` は `Offline` に `#[default]` を付けて `Default` を derive しているので、
代わりに入るのは `Offline` です。これで `match` は元の状態を所有し、その中身を新しい状態へムーブできます。
`match` 全体は `self.state = ...` の右辺のままなので、どのアームも次の状態を返さなければならず、
変わらない状態は `other => other` で元に戻ります。同じように、`mem::take(&mut self.closed)` は
一覧を返し、空の `Vec` を残します。

`send` はもう一方の場合の例です。遷移が中身をその場で変えるだけなら、`&mut self.state` で `match` すれば十分です。

`take` には落とし穴があります。`match` を文に書き換えて、一部のアームでしか `self.state` に代入しないと、
ほかのアームでは代わりに入れた値が残ります。オンラインのセッションや失敗したクライアントが、気づかないうちに
`Offline` になってしまいます。

### 別解

`std::mem::replace` は代わりに入れる値を引数で受け取るので、`Default` のない型にも使えます。
`drain(..)` は `Vec` を空にしながら要素を取り出します。

```rust
pub fn step(&mut self, ok: bool) {
    self.state = match std::mem::replace(&mut self.state, Conn::Offline) {
        Conn::Offline => Conn::Connecting { attempts: 1, queue: Vec::new() },
        Conn::Connecting { attempts, queue } if ok => {
            Conn::Online(Session { attempts, sent: queue })
        }
        Conn::Connecting { attempts, queue } if attempts < MAX_ATTEMPTS => {
            Conn::Connecting { attempts: attempts + 1, queue }
        }
        Conn::Connecting { queue, .. } => Conn::Failed { unsent: queue },
        Conn::Online(session) if !ok => {
            self.closed.push(session);
            Conn::Offline
        }
        other => other,
    };
}

pub fn take_closed(&mut self) -> Vec<Session> {
    self.closed.drain(..).collect()
}
```

その場で変更し、ムーブが必要なものだけを取り出す方法もあります。試行回数は `u32` なので `*attempts += 1` で
参照越しに変えられ、`mem::take(queue)` でキューだけを取り出せます。`Session` にはデフォルト値がないので、
セッションを閉じるときは状態全体を取り出します。

```rust
pub fn step(&mut self, ok: bool) {
    match &mut self.state {
        Conn::Offline => self.state = Conn::Connecting { attempts: 1, queue: Vec::new() },
        Conn::Connecting { attempts, queue } => {
            if ok {
                let sent = std::mem::take(queue);
                self.state = Conn::Online(Session { attempts: *attempts, sent });
            } else if *attempts < MAX_ATTEMPTS {
                *attempts += 1;
            } else {
                let unsent = std::mem::take(queue);
                self.state = Conn::Failed { unsent };
            }
        }
        Conn::Online(_) if !ok => {
            if let Conn::Online(session) = std::mem::take(&mut self.state) {
                self.closed.push(session);
            }
        }
        Conn::Online(_) | Conn::Failed { .. } => {}
    }
}
```
