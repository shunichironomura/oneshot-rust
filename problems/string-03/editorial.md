A `&str` is sliced by **byte** offsets into its UTF-8 encoding, and one character takes 1 to 4
bytes. `&s[..n]` therefore counts bytes, not characters, and it panics when `n` falls inside a
character (`byte index 2 is not a char boundary; it is inside 'é'`) or past the end of `s`. Every
function here has to turn a character count or a byte limit into a valid byte offset first, and
slice only then.

`char_indices` yields each character together with the byte offset where it starts. In
`first_chars`, the offset where character number `n` (counting from 0) starts is exactly where the
first `n` characters end, and when there are not that many characters, `map_or(s.len(), ...)` takes
all of `s`. `last_chars` walks from the back with `rev()`: the last of the `n` characters visited
starts the suffix. `take(n).last()` handles `n = 0` too: nothing is visited, so the result is the
empty slice at the end of `s`. The tempting `rev().nth(n - 1)` panics there, because `0 - 1`
overflows a `usize`.

`str::floor_char_boundary` (stable since Rust 1.91) rounds a byte index down to the nearest
character boundary, and returns `s.len()` for an index past the end, which is exactly what
`truncate_bytes` needs. In `ellipsize`, compare `s.len()` (bytes), not `s.chars().count()`, and
reserve 3 bytes for `'…'`: it is one `char` but `'…'.len_utf8()` is 3.

With one reference parameter, lifetime elision ties the returned `&str` to `s`, so slicing `s` is
all it takes; nothing is copied. A literal `""` would have the right value for an empty result, but
it points somewhere else, which is why every empty result here is still made by slicing `s`.

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

`last_chars` can count the characters first and skip all but the last `n`, reusing `first_chars` to
find where the skipped part ends. `saturating_sub` gives 0 instead of underflowing when `n` is larger
than the count.

```rust
pub fn last_chars(s: &str, n: usize) -> &str {
    let skip = s.chars().count().saturating_sub(n);
    &s[first_chars(s, skip).len()..]
}
```

Adding up `len_utf8` of the first (or last) `n` characters gives the byte length of the prefix (or
suffix) directly. For `n = 0` the sum is 0, so `last_chars` returns `&s[s.len()..]`.

```rust
pub fn first_chars(s: &str, n: usize) -> &str {
    let len: usize = s.chars().take(n).map(char::len_utf8).sum();
    &s[..len]
}

pub fn last_chars(s: &str, n: usize) -> &str {
    let len: usize = s.chars().rev().take(n).map(char::len_utf8).sum();
    &s[s.len() - len..]
}
```
