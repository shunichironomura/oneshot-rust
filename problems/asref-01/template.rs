pub fn char_len(s: &str) -> usize {
    s.as_ref().chars().count()
}

pub fn join_upper(words: &[String], sep: &str) -> String {
    let mut out = String::new();
    for (i, w) in words.iter().enumerate() {
        if i > 0 {
            out.push_str(sep);
        }
        out.push_str(&w.as_ref().to_uppercase());
    }
    out
}

pub fn total_chars(groups: Vec<Vec<String>>) -> usize {
    groups
        .into_iter()
        .map(|g| g.into_iter().map(|s| s.as_ref().chars().count()).sum::<usize>())
        .sum()
}

pub fn square_in_place(n: &mut u32) {
    let v = n.as_mut();
    *v = *v * *v;
}
