Software versions such as `1.10.2` or `2.0.0-rc.1` have a small grammar. Implement `FromStr` for
the type `Version` below, so that `s.parse::<Version>()` accepts exactly the valid strings. The type
is already in the editor, together with `ParseVersionError`.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseVersionError;

impl FromStr for Version { /* type Err = ParseVersionError; */ }
```

A valid string has the form `MAJOR[.MINOR[.PATCH]][-PRE]`, where the parts in brackets are optional.

- `MAJOR`, `MINOR` and `PATCH` are each one or more ASCII digits (`0` to `9`), and their value must
  fit in a `u64`. Leading zeros are allowed: `007` is 7. A missing `MINOR` or `PATCH` is 0, and
  without `-PRE` the field `pre` is `None`.
- `PRE`, the pre-release label, is one or more characters, each of which is an ASCII letter, an
  ASCII digit or `.`. It is stored in `pre` exactly as written.
- Nothing else is allowed. The string is not trimmed, and any other character or any empty part
  makes it invalid. For an invalid string, return `Err(ParseVersionError)`.
- Do not change the definitions of `Version` and `ParseVersionError`.

### Examples

```rust
"1.2".parse::<Version>()      // Ok(Version { major: 1, minor: 2, patch: 0, pre: None })
"3-rc.1".parse::<Version>()   // Ok(Version { major: 3, minor: 0, patch: 0, pre: Some("rc.1") })
"1.2.3.4".parse::<Version>()  // Err(ParseVersionError)
```

`""`, `"1..2"`, `"1.2-"` and `"v1"` are all invalid too.
