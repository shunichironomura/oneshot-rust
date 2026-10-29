/// The rounds (counted from 0) whose score is above `par`.
pub fn rounds_over(scores: &[u32], par: u32) -> Vec<usize> {
    scores
        .iter()
        .enumerate()
        .filter(|&(_, &s)| s > par)
        .map(|(round, _)| round)
        .collect()
}

/// The initials, in order.
pub fn initials(entries: &[(char, u32)]) -> String {
    entries.iter().map(|&(c, _)| c).collect()
}
