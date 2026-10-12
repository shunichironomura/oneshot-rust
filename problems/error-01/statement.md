Read a settings file made of `key = value` lines into a map, and report what is wrong when it cannot
be read.

```rust
#[derive(Debug, PartialEq)]
pub enum ConfigError {
    MissingEquals { line: usize },
    BadNumber(ParseIntError),
    Duplicate(String),
}

impl fmt::Display for ConfigError { /* to implement */ }

pub fn parse_config(text: &str) -> Result<HashMap<String, u32>, ConfigError>
```

- Every line is split at its **first** `=`. The key and the value are trimmed. The value must parse
  as a `u32`.
- Line numbers start at 1.
- On the first problem, return the matching error:

| Problem | Error | `to_string()` |
| --- | --- | --- |
| No `=` on line 3 | `MissingEquals { line: 3 }` | `line 3: missing '='` |
| Value is not a `u32` | `BadNumber(e)`, where `e` is the error of `parse` | `bad number: <e>` |
| Key `port` appears twice | `Duplicate("port")` | `duplicate key: port` |

`<e>` is the message of the `ParseIntError`, for example `invalid digit found in string`.

### Examples

```rust
parse_config("port = 8080\nworkers=4\n")
// Ok({"port": 8080, "workers": 4})

parse_config("port = 8080\nverbose\n")
// Err(MissingEquals { line: 2 }), displayed as "line 2: missing '='"
```
