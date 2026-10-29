A `str` is always valid UTF-8, so `\x` escapes in a `str` literal only go up to `\x7F`. `\u{89}`
compiles, but it is the character U+0089, which UTF-8 encodes as the two bytes `0xC2 0x89`, so the
result would have 9 bytes. A byte string `b"..."` holds bytes, not characters, and accepts `\x00` to
`\xFF`. Its type is `&'static [u8; 8]`, which coerces to the return type `&'static [u8]`. Drop the
`.as_bytes()` as well: it is a `str` method, and an array has none, so `b"...".as_bytes()` does not
compile.

### Alternatives

The bytes can be an array of `u8` literals, where `b'P'` is the byte of the ASCII character `P`. An
array of constants behind `&` is promoted to a `'static` value, so it can be returned as
`&'static [u8]`.

```rust
pub fn png_magic() -> &'static [u8] {
    &[0x89, b'P', b'N', b'G', b'\r', b'\n', 0x1A, b'\n']
}
```

Raw strings and byte strings can be combined: `br"C:\new"` is a byte string without escapes.
