use std::collections::HashMap;

use crate::judge::{case, check_eq};
use crate::user::letter_counts;

fn words(xs: &[&str]) -> Vec<String> {
    xs.iter().map(|s| s.to_string()).collect()
}

/// Runs `letter_counts` and returns the map as a list sorted by char, with the counter.
fn run_sorted(xs: &[&str], workers: usize) -> (Vec<(char, usize)>, usize) {
    let (map, processed): (HashMap<char, usize>, usize) = letter_counts(words(xs), workers);
    let mut list: Vec<(char, usize)> = map.into_iter().collect();
    list.sort();
    (list, processed)
}

pub fn run() {
    case("example-1", || check_eq(run_sorted(&["ab", "b", ""], 2), (vec![('a', 1), ('b', 2)], 3)));
    case("case-1", || {
        // No words at all: the worker has nothing to do.
        check_eq(letter_counts(Vec::new(), 1), (HashMap::new(), 0))
    });
    case("case-2", || {
        // More workers than words: worker 1 gets nothing.
        check_eq(run_sorted(&["yxy"], 2), (vec![('x', 1), ('y', 2)], 1))
    });
    case("case-3", || {
        // A single worker does everything; empty words are still processed.
        check_eq(run_sorted(&["", "noon", "", "on"], 1), (vec![('n', 3), ('o', 3)], 4))
    });
    case("case-4", || {
        // Every char counts as it is: case, punctuation, spaces and non-ASCII chars.
        let list = ["é", "Aa!", "日 é", "日é", "", "a"];
        let expected = vec![(' ', 1), ('!', 1), ('A', 1), ('a', 2), ('é', 3), ('日', 2)];
        check_eq(run_sorted(&list, 2), (expected, 6))
    });
}
