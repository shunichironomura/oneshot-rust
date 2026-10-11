pub fn first_chars(s: &str, n: usize) -> &str {
    // The byte offset where the (n + 1)-th char starts is where the prefix ends.
    let end = s.char_indices().nth(n).map_or(s.len(), |(i, _)| i);
    &s[..end]
}

pub fn last_chars(s: &str, n: usize) -> &str {
    // Walk back over at most n chars; the last one visited is where the suffix starts.
    let start = s.char_indices().rev().take(n).last().map_or(s.len(), |(i, _)| i);
    &s[start..]
}

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
