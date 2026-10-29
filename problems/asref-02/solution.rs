pub fn total_chars<I>(groups: I) -> usize
where
    I: IntoIterator<Item: IntoIterator<Item: AsRef<str>>>,
{
    groups
        .into_iter()
        .map(|g| g.into_iter().map(|s| s.as_ref().chars().count()).sum::<usize>())
        .sum()
}
