pub fn map_lines<F>(text: &str, f: F) -> Vec<String>
where
    F: Fn(&str) -> &str,
{
    let normalized = text.replace("\r\n", "\n");
    normalized.lines().map(|line| f(line).to_string()).collect()
}
