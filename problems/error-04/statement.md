A server reads its settings from `(key, value)` pairs. When a setting is wrong, the error keeps its
cause: a bad port wraps the `ParseIntError` that explains why the number did not parse, and the
caller can follow `Error::source` from an error to its cause.

```rust
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    Missing(&'static str),
    BadPort { value: String, source: ParseIntError },
}

impl fmt::Display for ConfigError { /* to implement */ }
impl Error for ConfigError { /* to implement */ }

pub fn parse_port(s: &str) -> Result<u16, ConfigError>
pub fn load(pairs: &[(&str, &str)]) -> Result<(String, u16, u64), Box<dyn Error>>
pub fn report(e: &dyn Error) -> String
```

`ConfigError` (the enum is already in the editor):

- `Missing(key)` is displayed as `missing key: <key>`, and `BadPort { value, .. }` as
  `bad port '<value>'`. The message of `BadPort` does not include the message of its
  `ParseIntError`.
- `source()` returns the `ParseIntError` stored in `BadPort`, and `None` for `Missing`.

`parse_port(s)` parses `s` as a `u16` with `str::parse`, without trimming it. If that fails, the
error is `BadPort { value, source }`, where `value` is `s` and `source` is the `ParseIntError`
returned by `parse`.

`load(pairs)`:

- The keys are checked in the following order, and the first problem found is returned:
  1. `host` is required; without it, the error is `Missing("host")`.
  2. `port` is required; without it, the error is `Missing("port")`. Its value is read with
     `parse_port`, and an error of `parse_port` is returned as it is.
  3. `timeout` is optional and defaults to `30`. Its value is parsed as a `u64` with `str::parse`,
     also without trimming it. If that fails, the error is the `ParseIntError` itself, not wrapped
     in a `ConfigError`.
- The result is `(host, port, timeout)`. Errors are returned boxed, and the tests check their type
  with `downcast_ref::<ConfigError>()` and `downcast_ref::<ParseIntError>()`.

`report(e)` works with any error, not only with the errors of this module. It returns the messages
(`to_string()`) of `e` and of every error in its source chain, in order, separated by `": "`.

### Examples

```rust
load(&[("host", "h"), ("port", "8080")])  // Ok(("h".to_string(), 8080, 30))
parse_port("70000")  // Err(BadPort { value: "70000", source: .. }), too large for a u16

let e = load(&[("host", "h"), ("port", "x1")]).unwrap_err();
e.to_string()                     // "bad port 'x1'"
e.downcast_ref::<ConfigError>()   // Some(BadPort { value: "x1", source: .. })
e.source().unwrap().to_string()   // "invalid digit found in string" (a ParseIntError)
report(&*e)                       // "bad port 'x1': invalid digit found in string"

let e = load(&[("host", "h"), ("port", "80"), ("timeout", "-1")]).unwrap_err();
e.downcast_ref::<ParseIntError>() // Some(..), and e.downcast_ref::<ConfigError>() is None
report(&*e)                       // "invalid digit found in string"
```
