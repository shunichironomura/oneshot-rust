pub fn initials(full_name: &str) -> String {
    let mut out = String::new();
    for word in full_name.split_whitespace() {
        out.push(word[0]);
        out.push('.');
    }
    out
}
