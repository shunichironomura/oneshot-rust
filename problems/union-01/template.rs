#[repr(C)]
union Bits32 {
    f: f32,
    u: u32,
}

#[repr(C)]
union ByteBool {
    byte: u8,
    flag: bool,
}

#[repr(C)]
union Widen {
    small: u8,
    big: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Header {
    kind: u8,
    len: u16,
}

#[repr(C)]
union HeaderBits {
    header: Header,
    bits: u32,
}

pub fn float_bits(x: f32) -> u32 {
    // SAFETY: `f32` and `u32` have the same size, and every bit pattern is a valid `u32`.
    unsafe { Bits32 { f: x }.u }
}

pub fn byte_to_bool(b: u8) -> Option<bool> {
    // SAFETY: `flag` is only used when `b` is 0 or 1, which are valid `bool`s.
    let flag = unsafe { ByteBool { byte: b }.flag };
    if b <= 1 { Some(flag) } else { None }
}

pub fn widen(b: u8) -> u32 {
    // SAFETY: `small` is the lowest byte of `big` (the target is little-endian), and every bit
    // pattern is a valid `u32`.
    unsafe { Widen { small: b }.big }
}

pub fn pack_header(kind: u8, len: u16) -> u32 {
    let mut packed = HeaderBits { bits: 0 };
    packed.header = Header { kind, len };
    // SAFETY: all four bytes were zeroed first, so the padding byte between `kind` and `len` is
    // 0, and every bit pattern is a valid `u32`.
    unsafe { packed.bits }
}
