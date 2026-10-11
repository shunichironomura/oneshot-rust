Read a settings file made of `key = value` lines into a map, and report what is wrong when it cannot
be read.

```rust
#[derive(Debug, PartialEq)]
pub enum ConfigError {
    MissingEquals { line: usize },
    EmptyKey { line: usize },
    BadNumber(ParseIntError),
    Duplicate(String),
}

impl fmt::Display for ConfigError { /* to implement */ }

pub fn parse_config(text: &str) -> Result<HashMap<String, u32>, ConfigError>
```

- Each line is trimmed first. Empty lines and lines starting with `#` are skipped.
- A line is split at its **first** `=`. The key and the value are trimmed. The value must parse as a
  `u32`.
- Line numbers start at 1 and count every line, including skipped ones.
- On the first problem, return the matching error:

| Problem | Error | `to_string()` |
| --- | --- | --- |
| No `=` on line 3 | `MissingEquals { line: 3 }` | `line 3: missing '='` |
| Empty key on line 3 | `EmptyKey { line: 3 }` | `line 3: empty key` |
| Value is not a `u32` | `BadNumber(e)`, where `e` is the error of `parse` | `bad number: <e>` |
| Key `port` appears twice | `Duplicate("port")` | `duplicate key: port` |

`<e>` is the message of the `ParseIntError`, for example `invalid digit found in string`.

### Examples

```rust
parse_config("# server\nport = 8080\n\nworkers=4\n")
// Ok({"port": 8080, "workers": 4})

parse_config("port = 8080\nverbose\n")
// Err(MissingEquals { line: 2 }), displayed as "line 2: missing '='"
```
