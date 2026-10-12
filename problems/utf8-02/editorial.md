`<[u8]>::utf8_chunks` splits the bytes into a valid `&str` (`valid()`) followed by one invalid
sequence (`invalid()`, which can be empty only in the last chunk). On its own it cannot tell an
incomplete character (`E2 82`) from an invalid sequence that nothing can complete (`FF`): both are
the `invalid()` part of the last chunk. Holding back every invalid sequence at the end would delay
the `'?'` for `FF`. `decode` tells them apart, but only for the sequence at the very end: `E2 82`
followed by `x` is invalid, although `decode(&[0xE2, 0x82])` alone says `Incomplete`.

The loop borrows `self.pending` through `utf8_chunks()`, so it only counts the decoded bytes in
`used`, and they are removed with `drain(..used)` after the loop. Whatever is left for `finish` is
an incomplete character or nothing, which `decode_lossy` turns into `"?"` or `""`.

### Alternatives

`push` can use `decode` alone: `Incomplete` is exactly the case where the bytes are held back.
`rest` borrows `self.pending`, so compute how many bytes were used before calling `drain`.

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
