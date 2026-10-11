Each function has one mistake, all three about how `String` and `str` work (Book chapter 8). They
are in different functions, so the compiler reports all three at once.

- **`word[0]`**: a `str` cannot be indexed by an integer. Its contents are UTF-8 bytes, and one
  character takes 1 to 4 of them, so "element 0" could mean a byte or a character, and Rust makes
  you say which. `word.chars().next()` returns the first `char` as an `Option`, which is `None` only
  for an empty string, and `split_whitespace` never yields an empty word. Two other fixes compile but
  are wrong: `word.as_bytes()[0] as char` takes the first byte of `é` (`0xC3`) and turns it into the
  character `Ã`, and `&word[..1]` panics, because a range on a `str` counts bytes and byte offset 1
  falls inside `é`, which is not a character boundary.
- **`dir + "/" + file`**: `+` on a `String` calls `add(self, s: &str) -> String`. It takes the left
  side by value, appends to its buffer and returns it, and the right side must be a `&str`. `"/"` is
  one, but `file` is a `String`. Write `&file`: the `&String` is deref-coerced to `&str`. Moving
  `dir` is fine, because it is not used afterwards.
- **`msg + "!"`**: for the same reason, `+` moves `msg`, so `msg.is_empty()` on the next line uses a
  moved value. Check first, then build the result. `msg.clone() + "!"` also compiles, but copies the
  string for nothing.

`to_uppercase` returns a new `String`, whose length may differ: `"straße"` becomes `"STRASSE"`.
Uppercasing `char` by `char` and keeping only the first character of each result would lose the
second `S`.

### Alternatives

`format!` only borrows its arguments, so it never moves anything; the Book recommends it for
combining several strings. The initials can also be collected from an iterator, because `String`
implements `FromIterator<String>`.

```rust
pub fn initials(full_name: &str) -> String {
    full_name
        .split_whitespace()
        .filter_map(|word| word.chars().next())
        .map(|first| format!("{first}."))
        .collect()
}

pub fn join_path(dir: String, file: String) -> String {
    if dir.is_empty() {
        file
    } else {
        format!("{dir}/{file}")
    }
}

pub fn shout(msg: String) -> String {
    if msg.is_empty() {
        String::new()
    } else {
        format!("{}!", msg.to_uppercase())
    }
}
```

`push` and `push_str` append in place, which is what `+` does under the hood. `to_uppercase` takes
`&self`, so `msg.to_uppercase() + "!"` does not move `msg`.

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

pub fn shout(msg: String) -> String {
    if msg.is_empty() {
        return String::new();
    }
    msg.to_uppercase() + "!"
}
```
