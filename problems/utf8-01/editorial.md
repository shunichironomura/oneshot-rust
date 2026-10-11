`std::str::from_utf8` checks the bytes and, if they are valid, returns a `&str` that points into
them, so nothing is copied. By lifetime elision the returned `&str` borrows `bytes`, which is
exactly what `from_utf8(bytes)` gives. `String::from_utf8(bytes.to_vec())` would make a new
`String`, and a `&str` into a local `String` cannot be returned.

On failure, the `Utf8Error` says where and what went wrong. `valid_up_to()` is the length of the
longest valid prefix, and `error_len()` is `Some(len)` for an invalid sequence of `len` bytes, or
`None` if the input ended in the middle of a character. These map one to one onto `Invalid` and
`Incomplete`.

For `decode_lossy`, `String::from_utf8_lossy` is tempting, but it returns a `Cow<str>`, and
`from_utf8_lossy(bytes).replace('\u{FFFD}', "?")` also rewrites every U+FFFD that was correctly
encoded in the input. Counting U+FFFD in its result has the same problem. `<[u8]>::utf8_chunks`
gives the pieces that `from_utf8_lossy` works with: each `Utf8Chunk` is a valid `&str`
(`valid()`) followed by one invalid sequence (`invalid()`, which can be empty only in the last
chunk). Writing your own decoder is harder than it looks: the length of an invalid sequence
depends on overlong encodings, surrogates and values above U+10FFFF, and replacing every bad byte
separately turns `F0 9F 98 41` into `"???A"` instead of `"?A"`.

In `Utf8Stream::push`, `utf8_chunks` alone cannot tell an incomplete character (`E2 82`) from an
invalid sequence that nothing can complete (`FF`): both are the `invalid()` part of the last chunk.
Holding back every invalid sequence at the end would delay the `'?'` for `FF`. `decode` tells them
apart, but only for the sequence at the very end: `E2 82` followed by `x` is invalid, although
`decode(&[0xE2, 0x82])` alone says `Incomplete`. The loop borrows `self.pending` through
`utf8_chunks()`, so the decoded bytes are removed with `drain(..used)` after the loop. Whatever is
left for `finish` is an incomplete character or nothing.

### Alternatives

`decode_lossy` with a loop over `from_utf8`. An incomplete character (`error_len()` is `None`)
runs to the end of the input, so the rest is skipped.

```rust
pub fn decode_lossy(bytes: &[u8]) -> (String, usize) {
    let mut text = String::with_capacity(bytes.len());
    let mut replaced = 0;
    let mut rest = bytes;
    loop {
        match std::str::from_utf8(rest) {
            Ok(valid) => {
                text.push_str(valid);
                return (text, replaced);
            }
            Err(e) => {
                let (valid, after) = rest.split_at(e.valid_up_to());
                // This prefix is valid UTF-8, so checking it again cannot fail.
                text.push_str(std::str::from_utf8(valid).unwrap());
                text.push('?');
                replaced += 1;
                rest = &after[e.error_len().unwrap_or(after.len())..];
            }
        }
    }
}
```

`push` can use `decode` in the same way: `Incomplete` is exactly the case where the bytes are held
back. `rest` borrows `self.pending`, so compute how many bytes were used before calling `drain`.

```rust
pub fn push(&mut self, chunk: &[u8]) -> String {
    self.pending.extend_from_slice(chunk);
    let mut text = String::new();
    let mut rest = &self.pending[..];
    loop {
        match decode(rest) {
            Ok(valid) => {
                text.push_str(valid);
                rest = &[];
                break;
            }
            Err(DecodeError::Invalid { at, len }) => {
                text.push_str(decode(&rest[..at]).unwrap());
                text.push('?');
                rest = &rest[at + len..];
            }
            Err(DecodeError::Incomplete { at }) => {
                text.push_str(decode(&rest[..at]).unwrap());
                rest = &rest[at..];
                break;
            }
        }
    }
    let used = self.pending.len() - rest.len();
    self.pending.drain(..used);
    text
}
```
