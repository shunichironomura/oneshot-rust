use std::cmp::Reverse;
use std::collections::HashMap;

pub fn mode(v: &[i32]) -> Option<i32> {
    let mut counts: HashMap<i32, usize> = HashMap::new();
    for &x in v {
        *counts.entry(x).or_insert(0) += 1;
    }
    counts
        .into_iter()
        .max_by_key(|&(value, count)| (count, Reverse(value)))
        .map(|(value, _)| value)
}
