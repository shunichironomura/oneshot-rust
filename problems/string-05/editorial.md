A `str` cannot be indexed by an integer (Book chapter 8). Its contents are UTF-8 bytes, and one
character takes 1 to 4 of them, so "element 0" could mean a byte or a character, and Rust makes you
say which. `word.chars().next()` returns the first `char` as an `Option`, which is `None` only for an
empty string, and `split_whitespace` never yields an empty word.

Two other fixes compile but are wrong:

- `word.as_bytes()[0] as char` takes the first byte of `é` (`0xC3`) and turns it into the character
  `Ã`.
- `&word[..1]` panics, because a range on a `str` counts bytes and byte offset 1 falls inside `é`,
  which is not a character boundary.

### Alternatives

`String` implements `FromIterator<String>`, so the initials can be collected from an iterator.

```rust
pub fn initials(full_name: &str) -> String {
    full_name
        .split_whitespace()
        .filter_map(|word| word.chars().next())
        .map(|first| format!("{first}."))
        .collect()
}
```
