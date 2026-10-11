pub fn matching_lines(text: &str, pattern: &str) -> impl Iterator<Item = &str> {
    let pattern = pattern.to_lowercase();
    text.lines()
        .filter(move |line| line.to_lowercase().contains(&pattern))
}
