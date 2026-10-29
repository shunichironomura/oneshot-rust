`+` on a `String` calls `add(self, s: &str) -> String` (Book chapter 8). It takes the left side by
value, appends to its buffer and returns it, and the right side must be a `&str`. `"/"` is one, but
`file` is a `String`. Write `&file`: the `&String` is deref-coerced to `&str`.

Since `+` moves its left side, `dir` cannot be used after `dir + ...`. That is fine here, because it
is not used afterwards; if you need it again, check or copy what you need first. `dir.clone() + ...`
also compiles, but copies the string for nothing.

### Alternatives

`format!` only borrows its arguments, so it never moves anything; the Book recommends it for
combining several strings.

```rust
pub fn join_path(dir: String, file: String) -> String {
    if dir.is_empty() {
        file
    } else {
        format!("{dir}/{file}")
    }
}
```

`push` and `push_str` append in place, which is what `+` does under the hood.

```rust
pub fn join_path(dir: String, file: String) -> String {
    if dir.is_empty() {
        return file;
    }
    let mut path = dir;
    path.push('/');
    path.push_str(&file);
    path
}
```
