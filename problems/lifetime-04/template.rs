pub fn strip_tag(line: &str, tag: &str) -> &str {
    match line.strip_prefix(tag) {
        Some(rest) => rest.trim_start(),
        None => line,
    }
}

pub fn pick(a: &str, b: &str, first: bool) -> &str {
    if first { a } else { b }
}

pub fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}
