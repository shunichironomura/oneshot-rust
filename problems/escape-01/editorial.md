In a normal string literal, `\` starts an escape, and only a few escapes exist: `\n`, `\r`, `\t`,
`\\`, `\0`, `\'`, `\"`, `\x41` and `\u{...}` (plus a `\` at the end of a line, which skips the line
break and the whitespace after it). `\d` and `\.` are errors, found while the source is read, before
any type checking. A raw string `r"..."` has no escapes at all, so every backslash is just a
backslash. It is the simplest way to write regular expressions and Windows paths.

Beware of fixing only the reported escapes in such text: the compiler cannot complain about
something that is still a valid literal. `\n` and `\t` are valid, so `"C:\new"` compiles but holds a
newline.

### Alternatives

The value can also be written with plain escapes, the way most languages write strings: double each
backslash.

```rust
pub fn number_pattern() -> &'static str {
    "\\d+\\.\\d+"
}
```
