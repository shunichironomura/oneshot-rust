A service writes its state to a log with `{:?}`, and passwords must never end up in the log.
Implement `Debug` by hand for `Credentials` (it is already in the editor).

```rust
pub struct Credentials {
    pub user: String,
    pub password: String,
    pub scopes: Vec<String>,
}

impl fmt::Debug for Credentials { /* to implement */ }
```

- `Credentials` prints exactly what `#[derive(Debug)]` would print for a struct named
  `Credentials` with the fields `user`, `password` and `scopes`, in this order. `user` and `scopes`
  are shown the way `{:?}` shows a `String` and a `Vec`.
- The value of `password` is always the bare text `<redacted>`, without quotes, whatever the
  password is (even an empty one).
- Like a derived `Debug`, the output follows the formatter's flags: the pretty form of `{:#?}`, and
  the extra indentation when the value is nested inside another value printed with `{:#?}`.
- Width, fill, alignment, precision and the `+` and `0` flags are not used.

### Examples

```rust
let c = Credentials {
    user: "ann".to_string(),
    password: "hunter2".to_string(),
    scopes: vec!["read".to_string(), "write".to_string()],
};
```

`format!("{c:?}")` is

```text
Credentials { user: "ann", password: <redacted>, scopes: ["read", "write"] }
```

and `format!("{c:#?}")` is

```text
Credentials {
    user: "ann",
    password: <redacted>,
    scopes: [
        "read",
        "write",
    ],
}
```
