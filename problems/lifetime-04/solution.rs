pub fn strip_tag<'a>(line: &'a str, tag: &str) -> &'a str {
    match line.strip_prefix(tag) {
        Some(rest) => rest.trim_start(),
        None => line,
    }
}

pub fn pick<'a>(a: &'a str, b: &'a str, first: bool) -> &'a str {
    if first { a } else { b }
}

pub fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}
