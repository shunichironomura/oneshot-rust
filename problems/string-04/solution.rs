pub fn last_chars(s: &str, n: usize) -> &str {
    // Walk back over at most n chars; the last one visited is where the suffix starts.
    let start = s.char_indices().rev().take(n).last().map_or(s.len(), |(i, _)| i);
    &s[start..]
}
