pub fn truncate_bytes(s: &str, max: usize) -> &str {
    &s[..s.floor_char_boundary(max)]
}

pub fn ellipsize(s: &str, max: usize) -> String {
    if s.len() <= max {
        return s.to_string();
    }
    // '…' takes 3 bytes in UTF-8, so keep at most max - 3 bytes of s.
    let mut out = truncate_bytes(s, max - 3).to_string();
    out.push('…');
    out
}
