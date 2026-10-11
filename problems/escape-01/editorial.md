Four different literal rules are broken here. The compiler reports most of the mistakes at once,
because they are found while the source is read, before any type checking. But it cannot complain
about something that is still a valid literal, and that is where the real trap is.

- In a normal string literal, `\` starts an escape, and only a few escapes exist: `\n`, `\r`, `\t`,
  `\\`, `\0`, `\'`, `\"`, `\x41` and `\u{...}`. `\d` and `\.` are errors. In `windows_path` the
  compiler reports only `\d`, because `\n` and `\t` are valid: a newline and a tab. If you fix only
  `\d`, the code compiles but returns `C:`, a newline, `ew`, a tab and `able\data.txt`: 19
  characters. A raw string `r"..."` has no escapes at all, so every backslash is just a backslash.
  It is the simplest way to write regular expressions and Windows paths.
- A `"` ends a normal literal, so `"{"name": ...` is the literal `"{"` followed by tokens the parser
  does not expect. (A word glued to a closing quote is read as a literal suffix, so `"{"name` is a
  single token, and the error points at the next one, `": "Ferris`.) Escape each quote as `\"`, or
  use a raw string `r#"..."#`, which may contain `"`.
- A raw string `r#"..."#` ends at the first `"#`, and `"#rust"` contains one. Add another `#`:
  `r##"..."##` ends only at `"##`, which the text does not contain. A backslash cannot help inside a
  raw string: `r#"... "\#rust" ..."#` compiles, but keeps the backslash.
- A `str` is always valid UTF-8, so `\x` escapes in a `str` literal only go up to `\x7F`. `\u{89}`
  compiles, but it is the character U+0089, which UTF-8 encodes as the two bytes `0xC2 0x89`, so
  the result would have 9 bytes. A byte string `b"..."` holds bytes, not characters, and accepts
  `\x00` to `\xFF`. Its type is `&'static [u8; 8]`, which coerces to the return type
  `&'static [u8]`. Drop the `.as_bytes()` as well: it is a `str` method, and an array has none, so
  `b"...".as_bytes()` does not compile.

### Alternatives

Every value can also be written with plain escapes, the way most languages write strings: double
each backslash and escape each quote. The bytes can be an array of `u8` literals, where `b'P'` is
the byte of the ASCII character `P`. An array of constants behind `&` is promoted to a `'static`
value, so it can be returned as `&'static [u8]`.

```rust
pub fn number_pattern() -> &'static str {
    "\\d+\\.\\d+"
}

pub fn windows_path() -> &'static str {
    "C:\\new\\table\\data.txt"
}

pub fn json_snippet() -> &'static str {
    "{\"name\": \"Ferris\", \"tags\": [\"crab\"]}"
}

pub fn hashtag_quote() -> &'static str {
    "He said \"#rust\" twice"
}

pub fn png_magic() -> &'static [u8] {
    &[0x89, b'P', b'N', b'G', b'\r', b'\n', 0x1A, b'\n']
}
```

Raw strings and byte strings can be combined: `br"C:\new"` is a byte string without escapes.
