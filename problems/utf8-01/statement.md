Bytes read from a file or a socket are not text yet: they become a `&str` only once they are known
to be valid UTF-8. Implement a strict decoder, a lossy decoder, and a decoder for bytes that arrive
in chunks. `DecodeError` and `Utf8Stream` are already in the editor.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    Invalid { at: usize, len: usize },
    Incomplete { at: usize },
}

pub fn decode(bytes: &[u8]) -> Result<&str, DecodeError>;
pub fn decode_lossy(bytes: &[u8]) -> (String, usize);

#[derive(Debug, Default)]
pub struct Utf8Stream {
    pending: Vec<u8>,
}

impl Utf8Stream {
    pub fn new() -> Self;
    pub fn push(&mut self, chunk: &[u8]) -> String;
    pub fn finish(self) -> String;
}
```

UTF-8 encodes every `char` in 1 to 4 bytes, and the bytes are read from the start, one character at
a time. When the bytes at some position are not the UTF-8 encoding of a `char`, the *invalid
sequence* at that position is:

- the longest run of bytes there that is the beginning of the encoding of some `char`, or
- the single byte there, if the encoding of no `char` begins with that byte.

Reading then continues right after the invalid sequence. These are exactly the sequences that
`String::from_utf8_lossy` replaces with U+FFFD, and each one is 1 to 3 bytes long.

| Bytes | Invalid sequences |
| --- | --- |
| `F0 9F 98 41` | `F0 9F 98`, the beginning of `😀` (`F0 9F 98 80`). Then `41` is read as `A`. |
| `C0 AF` | `C0` and `AF`: no `char` is encoded with a first byte `C0` or `AF`. |
| `ED A0 80` | `ED`, `A0` and `80`: `ED A0 80` would be the surrogate U+D800, which is not a `char`. |

An invalid sequence of the first kind that reaches the end of the input is an *incomplete
character*: more bytes could still complete it. For example, `E2 82` at the end is an incomplete
character (`€` is `E2 82 AC`), but `C0` at the end is not, because nothing can complete it.

`decode(bytes)` checks the bytes strictly.

- If all of `bytes` is valid UTF-8, it returns `Ok` with the text (an empty input is valid). The
  text borrows `bytes`; nothing is copied.
- Otherwise, let `at` be where the first invalid sequence starts, so that `bytes[..at]` is the
  longest valid prefix. It returns `Err(DecodeError::Incomplete { at })` if that sequence is an
  incomplete character, and `Err(DecodeError::Invalid { at, len })` otherwise, where `len` is the
  length of the sequence in bytes.

`decode_lossy(bytes)` never fails.

- It returns the text with every invalid sequence (including an incomplete character at the end)
  replaced by one `'?'`, together with the number of replacements.
- Everything else is kept as it is, including `'?'` and `'\u{FFFD}'` (U+FFFD, `EF BF BD` in UTF-8)
  when they are correctly encoded in `bytes`. They are not replacements and are not counted. In
  other words, the text is what `String::from_utf8_lossy` returns, except that each U+FFFD that it
  puts in place of an invalid sequence becomes `'?'`.

`Utf8Stream` decodes bytes that arrive in chunks, such as reads from a socket. A chunk may end in
the middle of a character.

- `new` creates a stream that has not received any bytes yet.
- `push(chunk)` adds `chunk` to the bytes received so far and returns the text decoded from them
  that has not been returned yet. If the bytes received so far (taken as the input) end with an
  incomplete character, its bytes are held back: they are not decoded yet, but read again together
  with the next chunk. Everything before them is decoded as in `decode_lossy`, so an invalid
  sequence becomes `'?'` as soon as it is received.
- `finish()` ends the stream. It returns `"?"` if bytes are still held back, and `""` otherwise.
- So, however the bytes are split into chunks (a chunk may be empty), joining the results of all
  the `push` calls and of `finish` gives the text of `decode_lossy` for all the bytes.
- You may change the private fields of `Utf8Stream`.

### Examples

```rust
decode("hé".as_bytes())                  // Ok("hé")
decode(b"hi\xFFthere")                   // Err(Invalid { at: 2, len: 1 })
decode(&[0xF0, 0x9F, 0x98, b'A'])        // Err(Invalid { at: 0, len: 3 })
decode(&[b'a', 0xE2, 0x82])              // Err(Incomplete { at: 1 })

decode_lossy(b"ab\xFFcd")                // ("ab?cd", 1)
decode_lossy(&[0xC0, 0xAF])              // ("??", 2)
decode_lossy(&[0xF0, 0x9F, 0x98, b'A'])  // ("?A", 1)
decode_lossy(b"\xEF\xBF\xBD?")           // ("\u{FFFD}?", 0): both are correctly encoded
decode_lossy(&[b'a', 0xE2, 0x82])        // ("a?", 1)

let mut s = Utf8Stream::new();
s.push(b"caf\xC3")        // "caf" (C3 is the beginning of 'é', C3 A9)
s.push(b"\xA9 \xE2")      // "é "
s.push(b"\x82\xAC!\xFF")  // "€!?" (nothing can complete FF)
s.finish()                // ""

let mut s = Utf8Stream::new();
s.push(&[b'x', 0xF0, 0x9F])  // "x"
s.finish()                   // "?"
```
