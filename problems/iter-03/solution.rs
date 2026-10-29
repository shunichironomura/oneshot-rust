pub fn last_long(words: &[&str], min: usize) -> Option<usize> {
    // `rposition` searches from the back but counts the index from the front.
    words.iter().rposition(|w| w.chars().count() >= min)
}
