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
