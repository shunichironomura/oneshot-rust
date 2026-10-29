A `&str` is sliced by **byte** offsets into its UTF-8 encoding, and one character takes 1 to 4
bytes. `&s[s.len() - n..]` therefore counts bytes, not characters, and it panics when the offset
falls inside a character (`byte index 2 is not a char boundary; it is inside 'é'`) or when `n` is
larger than `s.len()`. The character count has to be turned into a valid byte offset first, and `s`
sliced only then.

`char_indices` yields each character together with the byte offset where it starts. `last_chars`
walks from the back with `rev()`: the last of the `n` characters visited starts the suffix, and when
there are fewer than `n` characters, the walk ends at the first one, so all of `s` is taken.
`take(n).last()` handles `n = 0` too: nothing is visited, so `map_or(s.len(), ...)` gives the empty
slice at the end of `s`. The tempting `rev().nth(n - 1)` panics there, because `0 - 1` overflows a
`usize`.

With one reference parameter, lifetime elision ties the returned `&str` to `s`, so slicing `s` is
all it takes; nothing is copied. A literal `""` would have the right value for an empty result, but
it points somewhere else, which is why the empty result is still made by slicing `s`.

### Alternatives

Count the characters first and skip all but the last `n`. `saturating_sub` gives 0 instead of
underflowing when `n` is larger than the count, and `nth(skip)` finds where the skipped part ends.

```rust
pub fn last_chars(s: &str, n: usize) -> &str {
    let skip = s.chars().count().saturating_sub(n);
    let start = s.char_indices().nth(skip).map_or(s.len(), |(i, _)| i);
    &s[start..]
}
```

Adding up `len_utf8` of the last `n` characters gives the byte length of the suffix directly. For
`n = 0` the sum is 0, so this returns `&s[s.len()..]`.

```rust
pub fn last_chars(s: &str, n: usize) -> &str {
    let len: usize = s.chars().rev().take(n).map(char::len_utf8).sum();
    &s[s.len() - len..]
}
```
