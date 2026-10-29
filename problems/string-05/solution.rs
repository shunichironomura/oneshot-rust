pub fn initials(full_name: &str) -> String {
    let mut out = String::new();
    for word in full_name.split_whitespace() {
        // The first character, not the first byte: `é` takes two bytes in UTF-8.
        if let Some(first) = word.chars().next() {
            out.push(first);
            out.push('.');
        }
    }
    out
}
