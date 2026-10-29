pub fn strip_tag<'a>(line: &'a str, tag: &str) -> &'a str {
    match line.strip_prefix(tag) {
        Some(rest) => rest.trim_start(),
        None => line,
    }
}
