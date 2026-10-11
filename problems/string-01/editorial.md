A `String` is UTF-8, so one character can take several bytes (`'é'` is two). Slicing with
`&word[..1]` would panic on `"élan"` because byte 1 is not a character boundary. Walking the string
with `chars()` avoids byte offsets altogether.

`char::to_uppercase` returns an iterator, not a `char`, because some characters uppercase to more
than one (`'ß'` → `"SS"`). `String::extend` appends all of them. `to_ascii_uppercase` would leave
`'é'` and `'ñ'` unchanged.

A flag remembers whether the next character starts a word, so runs of whitespace are copied as is.
`s.split_whitespace()` followed by `join(" ")` would lose tabs, newlines and repeated spaces.

### Alternatives

`split_inclusive` cuts the string after each whitespace character, so every piece starts either with
the first character of a word or with whitespace (whose uppercase is itself).

```rust
pub fn capitalize_words(s: &str) -> String {
    s.split_inclusive(char::is_whitespace)
        .map(|piece| {
            let mut chars = piece.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                None => String::new(),
            }
        })
        .collect()
}
```
