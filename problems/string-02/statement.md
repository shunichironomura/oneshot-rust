The code below does not compile. **Without looking at the error message**, predict where the compiler
complains and fix it.

The three functions below are small string helpers.

```rust
pub fn initials(full_name: &str) -> String
pub fn join_path(dir: String, file: String) -> String
pub fn shout(msg: String) -> String
```

- `initials` returns the first character of each word of `full_name`, each followed by a `.`, in
  order. Words are separated by any amount of whitespace (spaces, tabs, newlines), and whitespace at
  the start or end is ignored. The characters are kept as they are, with no change of case, and a
  word may start with any Unicode character, not only an ASCII letter. If there is no word, the
  result is `""`.
- `join_path` returns `dir`, then a `/`, then `file`. If `dir` is empty, it returns `file`
  unchanged. Nothing else is special: an empty `file` still gets the `/` (`"usr"` and `""` give
  `"usr/"`).
- `shout` returns `""` if `msg` is empty. Otherwise it returns `msg` in uppercase, followed by a `!`.
  Uppercasing follows Unicode rules, like `str::to_uppercase`, so `ß` becomes `SS`.

Do not change the signatures.

### Examples

- `initials("Grace Brewster Hopper")` is `"G.B.H."`, and `initials("émile zola")` is `"é.z."`.
- `join_path("usr".into(), "bin".into())` is `"usr/bin"`.
- `shout("hey".into())` is `"HEY!"`.
