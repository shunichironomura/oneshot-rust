pub fn initials(full_name: &str) -> String {
    let mut out = String::new();
    for word in full_name.split_whitespace() {
        out.push(word[0]);
        out.push('.');
    }
    out
}

pub fn join_path(dir: String, file: String) -> String {
    if dir.len() == 0 {
        return file;
    }
    dir + "/" + file
}

pub fn shout(msg: String) -> String {
    let loud = msg + "!";
    if msg.is_empty() {
        return String::new();
    }
    loud.to_uppercase()
}
