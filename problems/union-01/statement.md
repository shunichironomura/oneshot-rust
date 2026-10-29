This code compiles, and it may even seem to work when you run it natively. But it contains
**undefined behavior (UB)**, and Miri detects it.

A small function packs a header into a `u32` by reinterpreting it through a union.

```rust
pub fn pack_header(kind: u8, len: u16) -> u32
```

- `pack_header` packs a header into one `u32`: `kind` in bits 0-7 (the lowest 8 bits), 0 in bits
  8-15, and `len` in bits 16-31. In other words, the result is `kind + len * 65536`.

Remove the UB so that the function works correctly for **every** input. (You may keep using
`unsafe` and the union if you like. The private types may be changed or removed.)

### Examples

`pack_header(0x12, 0x3456)` is `0x3456_0012`.
