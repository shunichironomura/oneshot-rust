`Closed` and `Open` hold no data; they exist only as type parameters. A struct must use every type
parameter in a field, so `PhantomData<State>` marks `State` as used without storing anything. It is
zero-sized, so `Connection<Open>` and `Connection<Closed>` have the same layout.

`open` and `close` take `self` **by value**. They move the host and the log into a new struct whose
type parameter is different, and the old value is gone, so a closed connection can never be used
after `open`. Since `send` exists only on `Connection<Open>`, sending on a closed connection is a
compile error instead of a runtime check. The two transitions share a private generic helper
`transition<Next>`, which can produce a connection in any state.

Struct update syntax (`Connection { _state: PhantomData, ..self }`) does not work here: it requires
the source and the result to have the same type, and `Connection<Closed>` and `Connection<Open>` are
different types.

### Alternatives

Since `Closed` and `Open` are zero-sized themselves, you can store the state directly instead of
`PhantomData`. This also costs no memory.

```rust
pub struct Connection<State> {
    host: String,
    log: Vec<String>,
    state: State,
}

impl Connection<Closed> {
    pub fn open(mut self) -> Connection<Open> {
        self.log.push("open".to_string());
        Connection {
            host: self.host,
            log: self.log,
            state: Open,
        }
    }
}
```
