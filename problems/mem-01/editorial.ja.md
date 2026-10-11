値で束縛するパターン（`queue`、`session`）を使って `match self.state` と書くと、
中身が `self.state` からムーブされます。しかし `self` は `&mut Client` にすぎません。
クライアントの持ち主は呼び出し側で、Rust は参照の先にある場所が空になることを許しません。
後でアームが新しい値を戻すとしても、その間だけ空にすることもできません
（途中でパニックが起きると、呼び出し側には状態がムーブされたあとのクライアントが残ってしまうからです）。
これが E0507 で、`take_closed` の `let closed = self.closed;` も同じ理由でエラーになります。
コンパイラの提案はここでは役に立ちません。`ref queue` や `&self.closed` は借用するだけですが、
次の状態はキューを**所有**しなければならず（`sent` や `unsent` になります）、閉じたセッションの一覧も
`Session` を所有しなければなりません。そして `Session` に `Clone` を実装することは禁じられています。

`std::mem::take(&mut self.state)` は、その場所にデフォルト値を入れ、元の状態を値として取り出すことを
1 回で行います。`Conn` は `Offline` に `#[default]` を付けて `Default` を derive しているので、
代わりに入るのは `Offline` です。`take_closed` ならこれで終わりです。後に残る空の `Vec` が、
まさに取り出した後の一覧のあるべき姿だからです。

`step` では、この代わりの値が罠になります。テンプレートの `match` を
`match std::mem::take(&mut self.state)` に変えるとコンパイルは通りますが、どれかのアームが次の状態を
格納するまで、クライアントは `Offline` になっています。そして最後のアーム `_ => {}` は何も格納しません。
「何も変えない」が「状態を失う」に変わってしまい、オンラインのクライアントは成功した `step` で
セッションを閉じないまま切断され、失敗したクライアントは `Offline` として生き返ります。
古い値を取り出したら、状態を変えない道も含めて、**すべての**道で状態を戻さなければなりません。

模範解答では、これを忘れようがない形にしています。`match` 全体が `self.state = ...` の右辺なので、
どのアームも次の状態を返さなければならず（そうでなければアームの型がそろいません）、
変わらない状態は `other => other` で元に戻ります。

`send` はもう一方の場合の例です。遷移が中身をその場で変えるだけなら、`&mut self.state` で `match` すれば
十分で、何も取り出す必要はありません。

### 別解

テンプレートの `match` 文の形を残し、最後のアームで変わらない状態を戻す方法です。
`std::mem::replace` は代わりに入れる値を引数で受け取るので、`Default` のない型にも使えます。
`drain(..)` は `Vec` を空にしながら要素を取り出します。

```rust
pub fn step(&mut self, ok: bool) {
    match std::mem::replace(&mut self.state, Conn::Offline) {
        Conn::Offline => {
            self.state = Conn::Connecting { attempts: 1, queue: Vec::new() };
        }
        Conn::Connecting { attempts, queue } => {
            self.state = if ok {
                Conn::Online(Session { attempts, sent: queue })
            } else if attempts < MAX_ATTEMPTS {
                Conn::Connecting { attempts: attempts + 1, queue }
            } else {
                Conn::Failed { unsent: queue }
            };
        }
        Conn::Online(session) if !ok => {
            self.closed.push(session);
            self.state = Conn::Offline;
        }
        other => self.state = other,
    }
}

pub fn take_closed(&mut self) -> Vec<Session> {
    self.closed.drain(..).collect()
}
```

その場で変更し、ムーブが必要なものだけを取り出す方法もあります。試行回数は `u32` なので `*attempts += 1` で
参照越しに変えられ、`mem::take(queue)` でキューだけを取り出せます。`Session` にはデフォルト値がないので、
セッションを閉じるときは状態全体を取り出します。この形では、変わらない状態は一度も取り出されないので、
最後の空のアームで正しく動きます。

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
