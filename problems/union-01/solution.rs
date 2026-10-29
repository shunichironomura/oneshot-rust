pub fn pack_header(kind: u8, len: u16) -> u32 {
    (u32::from(len) << 16) | u32::from(kind)
}
