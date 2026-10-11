use std::cmp::Reverse;
use std::collections::HashMap;

pub fn median(v: &[i32]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    let mut sorted = v.to_vec();
    sorted.sort_unstable();
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 1 {
        Some(f64::from(sorted[mid]))
    } else {
        Some((f64::from(sorted[mid - 1]) + f64::from(sorted[mid])) / 2.0)
    }
}

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
