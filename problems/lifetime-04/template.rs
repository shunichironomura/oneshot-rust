pub fn strip_tag(line: &str, tag: &str) -> &str {
    match line.strip_prefix(tag) {
        Some(rest) => rest.trim_start(),
        None => line,
    }
}
