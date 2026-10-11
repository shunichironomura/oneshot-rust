pub fn runs(v: &[i32]) -> Vec<(i32, usize)> {
    v.chunk_by(|a, b| a == b)
        .map(|group| (group[0], group.len()))
        .collect()
}

pub fn rising_runs(v: &[i32]) -> Vec<&[i32]> {
    v.chunk_by(|a, b| a < b).collect()
}

pub fn local_peaks(v: &[i32]) -> Vec<usize> {
    v.windows(3)
        .enumerate()
        .filter(|(_, w)| w[0] < w[1] && w[1] > w[2])
        .map(|(i, _)| i + 1)
        .collect()
}

pub fn pairs(v: &[i32]) -> (Vec<(i32, i32)>, Option<i32>) {
    let chunks = v.chunks_exact(2);
    let rest = chunks.remainder().first().copied();
    let pairs = chunks.map(|pair| (pair[0], pair[1])).collect();
    (pairs, rest)
}
