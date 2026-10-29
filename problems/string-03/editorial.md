A `&str` is sliced by **byte** offsets into its UTF-8 encoding, and one character takes 1 to 4
bytes. `&s[..max]` panics when `max` falls inside a character
(`byte index 2 is not a char boundary; it is inside 'é'`) or past the end of `s`, so the byte limit
has to be turned into a valid byte offset first, and `s` sliced only then.

`str::floor_char_boundary` (stable since Rust 1.91) rounds a byte index down to the nearest
character boundary, and returns `s.len()` for an index past the end, which is exactly what
`truncate_bytes` needs. In `ellipsize`, compare `s.len()` (bytes), not `s.chars().count()`, and
reserve 3 bytes for `'…'`: it is one `char` but `'…'.len_utf8()` is 3.

With one reference parameter, lifetime elision ties the returned `&str` to `s`, so slicing `s` is
all it takes; nothing is copied. A literal `""` would have the right value for an empty result, but
it points somewhere else, which is why an empty result here is still made by slicing `s`.

### Alternatives

Without `floor_char_boundary`, step back from `max` until `is_char_boundary` says yes. Index 0 is
always a boundary, so the loop ends.

```rust
pub fn truncate_bytes(s: &str, max: usize) -> &str {
    if max >= s.len() {
        return s;
    }
    let mut end = max;
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}
```
