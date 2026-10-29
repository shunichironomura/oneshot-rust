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

pub fn pack_header(kind: u8, len: u16) -> u32 {
    let mut packed = HeaderBits { bits: 0 };
    packed.header = Header { kind, len };
    // SAFETY: all four bytes were zeroed first, so the padding byte between `kind` and `len` is
    // 0, and every bit pattern is a valid `u32`.
    unsafe { packed.bits }
}
