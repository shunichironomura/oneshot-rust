The signature of `Error::source` is `fn source(&self) -> Option<&(dyn Error + 'static)>`. The
`'static` is not about the reference, which lives only as long as `&self`. It says that the *type*
of the cause holds no borrowed data, and only such types can be downcast: `downcast_ref` is defined
on `dyn Error + 'static`, because it compares `TypeId`s, which exist only for `'static` types. The
tempting `Option<&dyn Error>` does not compile. There the elided object lifetime becomes the
lifetime of `&self`, which promises less than the trait does, and the compiler reports a puzzling
E0803 ("cannot infer an appropriate lifetime"). Inside the method, `match self` binds `source` as a
`&ParseIntError`, and `Some(source)` coerces it to `&(dyn Error + 'static)`.

`BadPort` displays only its own message. The cause is reachable through `source()`, so code that
prints a whole chain, like `report`, would otherwise show it twice. This is the convention of the
standard library: an error either returns its cause from `source()` or includes it in its message,
not both.

In `load`, `?` converts each error with `From::from`, and std has
`impl<E: Error> From<E> for Box<dyn Error>`. So a `ConfigError` (which qualifies only with `Debug`,
`Display` and the `Error` impl) and a `ParseIntError` both go into the box as they are. The box
hides the type but keeps the value: `downcast_ref` gets it back, and that is how the tests tell a
`ConfigError` from a plain `ParseIntError`. Turning the timeout error into a message with
`map_err(|e| e.to_string())?` would keep the text but lose the type.

The helper `get` needs a named lifetime: its parameters contain several elided lifetimes, so the
compiler cannot tell which one the result borrows from (E0106). The result borrows from the values
(`&'a str`), not from `pairs` or `key`. `iter().find` passes a `&&(&str, &str)` to its closure, and
the pattern `&&(k, _)` copies the key out. With `|(k, _)|`, `k` would be a `&&str`, and `k == key`
does not compile (`&str` cannot be compared with `str`).

In `report`, the loop moves `cur` down the chain: `cur.source()` returns a reference that lives as
long as `cur`, so `cur = next` is fine. `report` takes a `&dyn Error` without `'static`: walking and
printing a chain needs no downcasting.

### Alternatives

`std::iter::successors` turns the chain into an iterator.

```rust
pub fn report(e: &dyn Error) -> String {
    std::iter::successors(Some(e), |&e| e.source())
        .map(|e| e.to_string())
        .collect::<Vec<_>>()
        .join(": ")
}
```

The `&` in `|&e|` matters. The closure receives a `&&dyn Error`, and std implements `Error` for
references too (`impl<T: Error + ?Sized> Error for &T`). With `|e| e.source()`, method lookup picks
that impl for the outer reference, so the result borrows the closure's argument and cannot be
returned ("lifetime may not live long enough"). `|&e|` copies the inner `&dyn Error` out first.

Recursion follows the chain as well.

```rust
pub fn report(e: &dyn Error) -> String {
    match e.source() {
        Some(cause) => format!("{e}: {}", report(cause)),
        None => e.to_string(),
    }
}
```

`load` can also pick the first value of each key in a single pass, and a `Result` alias shortens
the signatures. Give the alias a default error type as below: with only
`type Result<T> = std::result::Result<T, Box<dyn Error>>`, the `Result<u16, ConfigError>` of
`parse_port` no longer compiles (E0107). `transpose` turns the `Option<Result<u64, _>>` of the
timeout into a `Result<Option<u64>, _>`, so that `?` can be applied.

```rust
type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

pub fn parse_port(s: &str) -> Result<u16, ConfigError> {
    s.parse().map_err(|source| ConfigError::BadPort {
        value: s.to_string(),
        source,
    })
}

pub fn load(pairs: &[(&str, &str)]) -> Result<(String, u16, u64)> {
    let (mut host, mut port, mut timeout) = (None, None, None);
    for &(key, value) in pairs {
        let slot = match key {
            "host" => &mut host,
            "port" => &mut port,
            "timeout" => &mut timeout,
            _ => continue,
        };
        slot.get_or_insert(value);
    }
    let host = host.ok_or(ConfigError::Missing("host"))?;
    let port = parse_port(port.ok_or(ConfigError::Missing("port"))?)?;
    let timeout = timeout.map(str::parse::<u64>).transpose()?.unwrap_or(30);
    Ok((host.to_string(), port, timeout))
}
```
