pub fn local_peaks(v: &[i32]) -> Vec<usize> {
    v.windows(3)
        .enumerate()
        .filter(|(_, w)| w[0] < w[1] && w[1] > w[2])
        .map(|(i, _)| i + 1)
        .collect()
}
