use std::collections::HashSet;

pub fn first_repeat<'a>(words: &[&'a str]) -> Option<&'a str> {
    let mut seen = HashSet::new();
    for &word in words {
        // `insert` returns `false` when the word is already in the set.
        if !seen.insert(word) {
            return Some(word);
        }
    }
    None
}

pub fn all_unique(words: &[&str]) -> bool {
    let mut seen = HashSet::new();
    words.iter().all(|word| seen.insert(word))
}
