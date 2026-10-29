/// The first 8 bytes of every PNG file.
pub fn png_magic() -> &'static [u8] {
    b"\x89PNG\r\n\x1a\n"
}
