Reading a union field reinterprets the bytes the union holds as the type of that field. That is
sound only if those bytes form a **valid value** of that type. Integers accept every bit pattern,
but not uninitialized bytes: reading uninitialized memory as an integer is UB, not "some unknown
number".

`Header` is `#[repr(C)]`, so `len` is aligned to offset 2, and the byte at offset 1 is **padding**.
Zeroing `bits` first does not help: `packed.header = Header { .. }` stores a whole `Header` value,
and a value does not carry its padding, so storing it makes that byte uninitialized again. Reading
`bits` then reads that byte. Build the number with shifts instead.

The solution needs no `unsafe`. Shifts and `u32::from` also do not depend on the byte order of the
target, while the union-based `pack_header` shown below relies on the target being little-endian.
A union-based version can support both byte orders by storing `len.to_le()` and converting the
result with `u32::from_le`.

### Alternatives

The union can stay if the read sees only initialized bytes. Write the fields of `Header` one by
one: writing a single field touches only the bytes of that field, so the zeroed padding byte stays
0. (This version is correct only on little-endian targets, such as the judge's x86_64.)

```rust
pub fn pack_header(kind: u8, len: u16) -> u32 {
    let mut packed = HeaderBits { bits: 0 };
    packed.header.kind = kind;
    packed.header.len = len;
    // SAFETY: the padding byte has not been written since it was zeroed.
    unsafe { packed.bits }
}
```

Another way to keep `HeaderBits` is to remove the padding: add an explicit field `pad: u8` between
`kind` and `len`, and construct `Header { kind, pad: 0, len }`.

`pack_header` can also be built from bytes, without any union.

```rust
pub fn pack_header(kind: u8, len: u16) -> u32 {
    let [lo, hi] = len.to_le_bytes();
    u32::from_le_bytes([kind, 0, lo, hi])
}
```
