This code compiles, and it may even seem to work when you run it natively. But it contains
**undefined behavior (UB)**, and Miri detects it.

Four small functions reinterpret values through unions.

```rust
pub fn float_bits(x: f32) -> u32
pub fn byte_to_bool(b: u8) -> Option<bool>
pub fn widen(b: u8) -> u32
pub fn pack_header(kind: u8, len: u16) -> u32
```

- `float_bits` returns the IEEE 754 bit pattern of `x`: the bits of `x`, read as a `u32`.
- `byte_to_bool` returns `Some(false)` for the byte `0`, `Some(true)` for `1`, and `None` for every
  other byte.
- `widen` returns `b` as a `u32` with the same value.
- `pack_header` packs a header into one `u32`: `kind` in bits 0-7 (the lowest 8 bits), 0 in bits
  8-15, and `len` in bits 16-31. In other words, the result is `kind + len * 65536`.

Remove the UB so that every function works correctly for **every** input. (You may keep using
`unsafe` and the unions if you like. The private types may be changed or removed.)

### Examples

- `float_bits(1.0)` is `0x3F80_0000`, and `float_bits(-0.0)` is `0x8000_0000`.
- `byte_to_bool(0)` is `Some(false)`, and `byte_to_bool(1)` is `Some(true)`.
- `widen(0xAB)` is `0xAB`.
- `pack_header(0x12, 0x3456)` is `0x3456_0012`.
