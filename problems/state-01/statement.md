Implement a `Connection` to a host. A connection is either closed or open, and the state is part of
its **type**: `Connection<Closed>` or `Connection<Open>`. The skeleton is in `template.rs`; fill in the
bodies and add fields to `Connection` as you need. Do not change the signatures.

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

- `new` creates a closed connection with an empty log.
- `open` appends `"open"` to the log, `send` appends `"send <message>"`, and `close` appends
  `"close"`.
- The log keeps everything since `new`, across any number of opens and closes.

### Example

```rust
let mut conn = Connection::new("db").open();
conn.send("hi");
let conn = conn.close();
```

`conn.host()` is then `"db"` and `conn.log()` is `["open", "send hi", "close"]`.
