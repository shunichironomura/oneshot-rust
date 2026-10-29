pub fn total_chars(groups: Vec<Vec<String>>) -> usize {
    groups
        .into_iter()
        .map(|g| g.into_iter().map(|s| s.as_ref().chars().count()).sum::<usize>())
        .sum()
}
