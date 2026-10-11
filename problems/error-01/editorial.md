`?` does more than return early: it converts the error with `From::from` first. With
`impl From<ParseIntError> for ConfigError`, `value.trim().parse()?` turns a `ParseIntError` into
`ConfigError::BadNumber` on its own. For the other cases, `ok_or(...)?` turns a missing `=` into an
error, and `split_once('=')` splits at the first `=` only.

`Display` is what `to_string()` and `{}` use. Match on `self` and `write!` one message per variant;
the inner `ParseIntError` has its own `Display`, so `{e}` prints its message.

`HashMap::insert` returns the previous value, which tells you whether the key was already there.

### Alternatives

Without a `From` impl, convert the error at the call site with `map_err`. A tuple variant's name is
also a function, so `ConfigError::BadNumber` can be passed directly.

```rust
let value: u32 = value.trim().parse().map_err(ConfigError::BadNumber)?;
```

This is handy when the same source error should become different variants in different places. When
it always maps to one variant, `From` keeps every call site short.
