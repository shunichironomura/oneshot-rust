The code below does not compile. Fix it.

Every PNG file starts with the same 8 bytes, its signature. The function returns them, written as a
literal in its body.

```rust
pub fn png_magic() -> &'static [u8]
```

The tests compare the result with the signature byte by byte. Do not change the signature of the
function.

### Examples

- `png_magic()` returns the 8 bytes `0x89`, `0x50`, `0x4E`, `0x47`, `0x0D`, `0x0A`, `0x1A`, `0x0A`.
