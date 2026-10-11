use std::collections::{BTreeSet, HashSet};

#[derive(Debug, PartialEq)]
pub struct Overlap {
    pub both: Vec<String>,
    pub only_left: Vec<String>,
    pub only_right: Vec<String>,
    pub exactly_one: Vec<String>,
    pub all: Vec<String>,
}

pub fn overlap(left: &[&str], right: &[&str]) -> Overlap {
    // A `BTreeSet` drops duplicates and iterates in ascending order.
    let left: BTreeSet<&str> = left.iter().copied().collect();
    let right: BTreeSet<&str> = right.iter().copied().collect();
    Overlap {
        both: owned(left.intersection(&right)),
        only_left: owned(left.difference(&right)),
        only_right: owned(right.difference(&left)),
        exactly_one: owned(left.symmetric_difference(&right)),
        all: owned(left.union(&right)),
    }
}

/// The set operations yield references to the elements (`&&str` here).
fn owned<'a>(words: impl Iterator<Item = &'a &'a str>) -> Vec<String> {
    words.map(|w| w.to_string()).collect()
}

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
