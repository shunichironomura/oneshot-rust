Reading a union field reinterprets the bytes the union holds as the type of that field. That is
sound only if those bytes form a **valid value** of that type. Integers and floats accept every bit
pattern, but not uninitialized bytes, and a `bool` accepts only `0` and `1`. Going through the four
functions:

- `float_bits` was already sound: all four bytes are written as an `f32`, and every bit pattern is a
  valid `u32`. It does what `f32::to_bits` does, which is the idiomatic way to write it. Do not
  replace it with `x as u32`: `as` converts the **number** (`1.0` becomes `1`), it does not keep the
  bits.
- `byte_to_bool` reads `flag` **before** it checks `b`. For `b = 2`, that read produces a `bool`
  holding `2`, and producing an invalid value is UB at that very moment, whether or not the value is
  used later. Since the read is only allowed when `b` is 0 or 1, the compiler may even conclude that
  `b <= 1` always holds and remove the check. A `match` on the byte needs no `unsafe` at all.
- `widen` writes one byte and reads four. The other three bytes were never written, and reading
  uninitialized memory as an integer is UB: it is not "some unknown number". `u32::from(b)` (or
  `b as u32`, which never loses anything here) converts the number.
- `pack_header` is the subtle one. `Header` is `#[repr(C)]`, so `len` is aligned to offset 2, and
  the byte at offset 1 is **padding**. Zeroing `bits` first does not help:
  `packed.header = Header { .. }` stores a whole `Header` value, and a value does not carry its
  padding, so storing it makes that byte uninitialized again. Reading `bits` then reads that byte.
  Build the number with shifts instead.

The solution needs no `unsafe`. Shifts and `u32::from` also do not depend on the byte order of the
target, while any union-based `widen` or `pack_header` relies on the target being little-endian.

### Alternatives

The unions can stay if every read sees only valid, initialized bytes. Read the `bool` after the
check, zero `big` before writing `small`, and write the fields of `Header` one by one: writing a
single field touches only the bytes of that field, so the zeroed padding byte stays 0. `float_bits`
stays as it was. (These versions of `widen` and `pack_header` are correct only on little-endian
targets, such as the judge's x86_64.)

```rust
pub fn byte_to_bool(b: u8) -> Option<bool> {
    if b <= 1 {
        // SAFETY: 0 and 1 are valid `bool`s.
        Some(unsafe { ByteBool { byte: b }.flag })
    } else {
        None
    }
}

pub fn widen(b: u8) -> u32 {
    let mut w = Widen { big: 0 };
    w.small = b;
    // SAFETY: all four bytes are initialized: `b` and three zeros.
    unsafe { w.big }
}

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
