There are two errors, and the compiler reports both at once. Both sit on the borders between `u8`,
`usize` and `char`, which Rust never crosses on its own.

- **`buckets[letter - b'a']`**: `letter` comes from the range `b'a'..=b'z'`, so it is a `u8`, and so
  is the difference. Arrays, slices and `Vec`s are indexed by `usize` only (the slice "cannot be
  indexed by `u8`"), and a smaller integer is not widened automatically.
  `usize::from(letter - b'a')` widens it without loss; `(letter - b'a') as usize` works too.
- **`letters.push(letter)`**: a `u8` is not a `char`, even when it holds an ASCII code.
  `char::from(letter)` or `letter as char` converts it. This conversion exists only from `u8`: every
  byte value is a valid `char` (U+0000 to U+00FF), but not every `u32` is, so `97u32 as char` is
  rejected, and `char::from_u32` returns an `Option`.

### Alternatives

A range of `char`s is an iterator too, so `letters_used` can pair each letter with its bucket, with
no arithmetic and no index at all.

```rust
pub fn letters_used(buckets: &[Vec<String>; 26]) -> String {
    ('a'..='z')
        .zip(buckets)
        .filter(|(_, bucket)| !bucket.is_empty())
        .map(|(letter, _)| letter)
        .collect()
}
```

`enumerate` gives the index as a `usize`, so only the way back to a `char` needs care. Convert the
index to `u8` before adding: `(b'a' + i) as char` adds a `usize` to a `u8`, and
`('a' as usize + i) as char` is rejected, because only a `u8` can be cast to `char`.

```rust
pub fn letters_used(buckets: &[Vec<String>; 26]) -> String {
    let mut letters = String::new();
    for (i, bucket) in buckets.iter().enumerate() {
        if !bucket.is_empty() {
            letters.push((b'a' + i as u8) as char);
        }
    }
    letters
}
```
