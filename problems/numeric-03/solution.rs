pub fn wrap_index(i: i64, len: usize) -> usize {
    // `len` is at most 1,000,000, so it fits in an `i64`.
    i.rem_euclid(len as i64) as usize
}
