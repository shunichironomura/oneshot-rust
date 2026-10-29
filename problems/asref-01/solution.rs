pub fn join_upper<S: AsRef<str>>(words: &[S], sep: &str) -> String {
    let mut out = String::new();
    for (i, w) in words.iter().enumerate() {
        if i > 0 {
            out.push_str(sep);
        }
        out.push_str(&w.as_ref().to_uppercase());
    }
    out
}
