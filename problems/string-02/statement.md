The code below does not compile. Fix it.

`join_path` joins a directory and a file name into one path.

```rust
pub fn join_path(dir: String, file: String) -> String
```

- It returns `dir`, then a `/`, then `file`.
- If `dir` is empty, it returns `file` unchanged.
- Nothing else is special: an empty `file` still gets the `/` (`"usr"` and `""` give `"usr/"`).

Do not change the signature.

### Examples

- `join_path("usr".into(), "bin".into())` is `"usr/bin"`.
