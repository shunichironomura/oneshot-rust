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
    OutOfRange(u32),
}

impl fmt::Display for ConfigError { /* to implement */ }
impl Error for ConfigError { /* to implement */ }

pub fn parse_port(s: &str) -> Result<u16, ConfigError>
pub fn load(pairs: &[(&str, &str)]) -> Result<(String, u16, u64), Box<dyn Error>>
pub fn root_cause<'a>(e: &'a (dyn Error + 'static)) -> &'a (dyn Error + 'static)
pub fn report(e: &dyn Error) -> String
```

`ConfigError` (the enum is already in the editor):

- `Missing(key)` is displayed as `missing key: <key>`, `BadPort { value, .. }` as
  `bad port '<value>'`, and `OutOfRange(n)` as `port <n> out of range`. The message of `BadPort`
  does not include the message of its `ParseIntError`.
- `source()` returns the `ParseIntError` stored in `BadPort`, and `None` for the other variants.

`parse_port(s)`:

- Parse `s` as a `u32` with `str::parse`, without trimming it. If that fails, the error is
  `BadPort { value, source }`, where `value` is `s` and `source` is the `ParseIntError` returned by
  `parse`.
- A number that is `0` or greater than `65535` gives `OutOfRange(n)`. Any other number is the port.

`load(pairs)`:

- Keys are compared exactly (case-sensitive, no trimming). Pairs with other keys are ignored. When a
  key appears more than once, only its first pair is used.
- The keys are checked in the following order, and the first problem found is returned:
  1. `host` is required; without it, the error is `Missing("host")`. Its value is used as it is,
     even if it is empty.
  2. `port` is required; without it, the error is `Missing("port")`. Its value is read with
     `parse_port`, and an error of `parse_port` is returned as it is.
  3. `timeout` is optional and defaults to `30`. Its value is parsed as a `u64` with `str::parse`,
     also without trimming it. If that fails, the error is the `ParseIntError` itself, not wrapped
     in a `ConfigError`.
- The result is `(host, port, timeout)`. Errors are returned boxed, and the tests check their type
  with `downcast_ref::<ConfigError>()` and `downcast_ref::<ParseIntError>()`.

`root_cause` and `report` work with any error, not only with the errors of this module:

- `root_cause(e)` follows `source()` from `e` for as long as there is one, and returns the last
  error of the chain. That is `e` itself if `e` has no source.
- `report(e)` returns the messages (`to_string()`) of `e` and of every error in its source chain, in
  order, separated by `": "`.

### Examples

```rust
load(&[("host", "h"), ("port", "8080")])  // Ok(("h".to_string(), 8080, 30))
parse_port("70000")                      // Err(ConfigError::OutOfRange(70000))

let e = load(&[("host", "h"), ("port", "x1")]).unwrap_err();
e.to_string()                     // "bad port 'x1'"
e.downcast_ref::<ConfigError>()   // Some(BadPort { value: "x1", source: .. })
root_cause(&*e).to_string()       // "invalid digit found in string" (a ParseIntError)
report(&*e)                       // "bad port 'x1': invalid digit found in string"

let e = load(&[("host", "h"), ("port", "80"), ("timeout", "-1")]).unwrap_err();
e.downcast_ref::<ParseIntError>() // Some(..), and e.downcast_ref::<ConfigError>() is None
report(&*e)                       // "invalid digit found in string"
```
