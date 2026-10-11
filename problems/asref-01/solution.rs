pub fn char_len<S: AsRef<str>>(s: S) -> usize {
    s.as_ref().chars().count()
}

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

pub fn total_chars<I>(groups: I) -> usize
where
    I: IntoIterator<Item: IntoIterator<Item: AsRef<str>>>,
{
    groups
        .into_iter()
        .map(|g| g.into_iter().map(|s| s.as_ref().chars().count()).sum::<usize>())
        .sum()
}

pub fn square_in_place<T: AsMut<u32>>(n: &mut T) {
    let v = n.as_mut();
    *v = *v * *v;
}

pub fn widen<I>(values: I) -> Vec<i64>
where
    I: IntoIterator<Item: Into<i64>>,
{
    values.into_iter().map(Into::into).collect()
}
