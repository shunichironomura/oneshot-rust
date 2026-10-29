The body of the function below is already written, but the code does not compile. Change **only the
signature** so that the test harness (the `tests.rs` tab) compiles and passes. Do not change the
body.

```rust
pub fn strip_tag(line: &str, tag: &str) -> &str
```

- `strip_tag` removes `tag` from the start of `line`, then skips the whitespace right after it, and
  returns the rest. Whitespace at the end is kept, and only one `tag` is removed. If `line` does not
  start with `tag` (case matters, and whitespace before the tag is not skipped), it returns `line`
  itself unchanged. The result is always a slice of `line`.

### Examples

- `strip_tag("[INFO]  server started", "[INFO]")` is `"server started"`.
