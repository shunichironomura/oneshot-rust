pub fn longest_word(s: &str) -> &str {
    s.split_whitespace()
        .fold("", |best, w| if w.len() > best.len() { w } else { best })
}
