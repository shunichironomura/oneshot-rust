次のコードはコンパイルが通りません。**エラーメッセージを見ずに**、どこで怒られるかを予想して直してください。

`Client` はネットワークの接続を保ちます。状態は `Conn` で表し、接続を試みている間に送ったメッセージは、
接続できるまでキューで待ちます。`Session` は確立した 1 回分の接続です。`attempts` は接続までにかかった試行回数、
`sent` はそのセッションのメッセージ（古い順）です。型はエディタに入っています。

```rust
const MAX_ATTEMPTS: u32 = 3;

#[derive(Debug, PartialEq)]
pub struct Session {
    pub attempts: u32,
    pub sent: Vec<String>,
}

#[derive(Debug, Default, PartialEq)]
pub enum Conn {
    #[default]
    Offline,
    Connecting { attempts: u32, queue: Vec<String> },
    Online(Session),
    Failed { unsent: Vec<String> },
}

#[derive(Default)]
pub struct Client {
    state: Conn,
    closed: Vec<Session>,
}

impl Client {
    pub fn new() -> Client;
    pub fn state(&self) -> &Conn;
    pub fn step(&mut self, ok: bool);
    pub fn send(&mut self, msg: &str) -> bool;
    pub fn take_closed(&mut self) -> Vec<Session>;
}
```

- `new` は、`Offline` で、閉じたセッションを 1 つも持たないクライアントを作ります。
  `state` は今の状態を返します。
- `step(ok)` は時間を 1 単位進めます。`ok` は、その間にネットワークが応答したかどうかです。
  状態は次のように変わります。
  - `Offline` は、`ok` にかかわらず `Connecting { attempts: 1, queue: [] }` になります。
  - `Connecting { attempts, queue }` は次のようになります。
    - `ok` なら `Online(Session { attempts, sent: queue })` になります。キューにあったメッセージが、
      そのセッションの最初のメッセージになります。
    - そうでなく `attempts < 3`（`MAX_ATTEMPTS`）なら、キューはそのままで
      `Connecting { attempts: attempts + 1, queue }` になります。
    - どちらでもなければ（3 回目の試行も失敗したら）`Failed { unsent: queue }` になります。
  - `Online(session)` は、`ok` なら変わりません。そうでなければ接続が切れたことになり、
    状態は `Offline` になって、`session` はクライアントの閉じたセッションの末尾に加わります。
  - `Failed { .. }` は最終状態で、`step` を呼んでも変わりません。
- `send(msg)` は、`Connecting` なら `msg` を `queue` の末尾に、`Online` ならセッションの `sent` の
  末尾に加えて、`true` を返します。`Offline` か `Failed` なら何も変えずに `false` を返します。
- `take_closed` は、閉じたセッションを閉じた順に返し、一覧を空にします。
  そのため、同じセッションが返るのは 1 回だけです。

型の定義（derive を含む）と公開されているシグネチャは変えず、
これらの型にトレイトを追加で実装することもしないでください。

### 例

```rust
let mut c = Client::new();
c.send("lost");  // false：オフライン
c.step(true);    // Connecting { attempts: 1, queue: [] }
c.send("hello"); // true
c.step(false);   // Connecting { attempts: 2, queue: ["hello"] }
c.step(true);    // Online(Session { attempts: 2, sent: ["hello"] })
c.send("bye");   // true
c.step(false);   // Offline
c.take_closed(); // [Session { attempts: 2, sent: ["hello", "bye"] }]
c.take_closed(); // []
```

新しいクライアントで `step(true)`、`send("x")` の後に `step(false)` を 3 回呼ぶと
`Failed { unsent: ["x"] }` になり、その後は `step` を呼んでも変わりません。
このとき `send("y")` は `false` を返します。
