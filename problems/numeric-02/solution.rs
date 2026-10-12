pub fn sum_wrapping(xs: &[u8]) -> u8 {
    xs.iter().fold(0, |total, &x| total.wrapping_add(x))
}

pub fn sum_checked(xs: &[u8]) -> Option<u8> {
    xs.iter().try_fold(0u8, |total, &x| total.checked_add(x))
}

pub fn charge(level: u8, amount: u8) -> u8 {
    level.saturating_add(amount).min(100)
}

pub fn wrap_index(i: i64, len: usize) -> usize {
    // `len` is at most 1,000,000, so it fits in an `i64`.
    i.rem_euclid(len as i64) as usize
}

pub fn page_count(items: u64, per_page: u64) -> u64 {
    items.div_ceil(per_page)
}

pub fn middle(a: u64, b: u64) -> u64 {
    a.midpoint(b)
}

pub fn distance(a: i64, b: i64) -> u64 {
    a.abs_diff(b)
}
