pub fn float_bits(x: f32) -> u32 {
    x.to_bits()
}

pub fn byte_to_bool(b: u8) -> Option<bool> {
    match b {
        0 => Some(false),
        1 => Some(true),
        _ => None,
    }
}

pub fn widen(b: u8) -> u32 {
    u32::from(b)
}

pub fn pack_header(kind: u8, len: u16) -> u32 {
    (u32::from(len) << 16) | u32::from(kind)
}
