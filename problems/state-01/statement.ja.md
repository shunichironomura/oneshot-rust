ホストへの接続 `Connection` を実装してください。接続は閉じているか開いているかのどちらかで、その状態は
**型**の一部です（`Connection<Closed>` または `Connection<Open>`）。骨組みは `template.rs` にあります。
本体を埋め、必要に応じて `Connection` にフィールドを追加してください。シグネチャは変えないでください。

```rust
impl Connection<Closed> {
    pub fn new(host: &str) -> Connection<Closed>
    pub fn open(self) -> Connection<Open>
}

impl Connection<Open> {
    pub fn send(&mut self, message: &str)
    pub fn close(self) -> Connection<Closed>
}

impl<State> Connection<State> {
    pub fn host(&self) -> &str
    pub fn log(&self) -> &[String]
}
```

- `new` は、ログが空の閉じた接続を作ります。
- `open` はログに `"open"` を、`send` は `"send <message>"` を、`close` は `"close"` を追加します。
- ログには、何度開け閉めしても `new` 以降のすべてが残ります。

### 例

```rust
let mut conn = Connection::new("db").open();
conn.send("hi");
let conn = conn.close();
```

このとき `conn.host()` は `"db"`、`conn.log()` は `["open", "send hi", "close"]` です。
