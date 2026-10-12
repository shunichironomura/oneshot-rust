Bytes read from a socket arrive in chunks, and a chunk may end in the middle of a character.
Implement `Utf8Stream`, which decodes such bytes as they arrive. `decode` and `decode_lossy` are
already implemented in the editor, and you may use them.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    Invalid { at: usize, len: usize },
    Incomplete { at: usize },
}

pub fn decode(bytes: &[u8]) -> Result<&str, DecodeError>; // given
pub fn decode_lossy(bytes: &[u8]) -> (String, usize);      // given

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

UTF-8 encodes every `char` in 1 to 4 bytes. When the bytes at some position are not the encoding of
a `char`, the *invalid sequence* there is the longest run of bytes that is the beginning of the
encoding of some `char`, or the single byte there if no encoding begins with it. These are exactly
the sequences that `String::from_utf8_lossy` replaces with U+FFFD. An invalid sequence of the first
kind that reaches the end of the input is an *incomplete character*: more bytes could still
complete it. `E2 82` at the end is one (`€` is `E2 82 AC`), but `C0` or `FF` at the end is not.

The given functions:

- `decode(bytes)` returns `Ok` with the text if all of `bytes` is valid. Otherwise it returns
  `Err(Incomplete { at })` if the first invalid sequence starts at `at` and is an incomplete
  character, and `Err(Invalid { at, len })` for any other invalid sequence of `len` bytes.
- `decode_lossy(bytes)` returns the text with every invalid sequence (including an incomplete
  character at the end) replaced by one `'?'`, and the number of replacements.

`Utf8Stream`:

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
let mut s = Utf8Stream::new();
s.push(b"caf\xC3")        // "caf" (C3 is the beginning of 'é', C3 A9)
s.push(b"\xA9 \xE2")      // "é "
s.push(b"\x82\xAC!\xFF")  // "€!?" (nothing can complete FF)
s.finish()                // ""

let mut s = Utf8Stream::new();
s.push(&[b'x', 0xF0, 0x9F])  // "x"
s.finish()                   // "?"
```
