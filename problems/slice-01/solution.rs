pub fn rising_runs(v: &[i32]) -> Vec<&[i32]> {
    v.chunk_by(|a, b| a < b).collect()
}
