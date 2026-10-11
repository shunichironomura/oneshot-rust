pub fn matching_lines<'a>(
    text: &'a str,
    pattern: &str,
) -> impl Iterator<Item = &'a str> + use<'a> {
    let pattern = pattern.to_lowercase();
    text.lines()
        .filter(move |line| line.to_lowercase().contains(&pattern))
}
