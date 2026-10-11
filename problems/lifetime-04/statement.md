The bodies of the three functions below are already written, but the code does not compile. Change
**only the signatures** so that the test harness (the `tests.rs` tab) compiles and passes. Do not
change the bodies.

```rust
pub fn strip_tag(line: &str, tag: &str) -> &str
pub fn pick(a: &str, b: &str, first: bool) -> &str
pub fn first_word(s: &str) -> &str
```

- `strip_tag` removes `tag` from the start of `line`, then skips the whitespace right after it, and
  returns the rest. Whitespace at the end is kept, and only one `tag` is removed. If `line` does not
  start with `tag` (case matters, and whitespace before the tag is not skipped), it returns `line`
  itself unchanged. The result is always a slice of `line`.
- `pick` returns `a` itself if `first` is `true`, and `b` itself otherwise.
- `first_word` returns the first word of `s`, where words are separated by whitespace (spaces, tabs,
  newlines, ...). The result is a slice of `s`. If `s` has no words, it returns `""`.

### Examples

- `strip_tag("[INFO]  server started", "[INFO]")` is `"server started"`.
- `pick("left", "right", true)` is `"left"`, and `pick("left", "right", false)` is `"right"`.
- `first_word("  hello world")` is `"hello"`, and `first_word("Rust is fun")` is `"Rust"`.
