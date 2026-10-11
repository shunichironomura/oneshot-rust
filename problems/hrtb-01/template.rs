pub fn map_lines<'a, F>(text: &'a str, f: F) -> Vec<String>
where
    F: Fn(&'a str) -> &'a str,
{
    let normalized = text.replace("\r\n", "\n");
    normalized.lines().map(|line| f(line).to_string()).collect()
}
