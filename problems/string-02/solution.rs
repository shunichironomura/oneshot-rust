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

pub fn join_path(dir: String, file: String) -> String {
    if dir.is_empty() {
        return file;
    }
    dir + "/" + &file
}

pub fn shout(msg: String) -> String {
    if msg.is_empty() {
        return String::new();
    }
    let loud = msg + "!";
    loud.to_uppercase()
}
