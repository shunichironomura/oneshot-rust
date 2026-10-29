pub fn sum_checked(xs: &[u8]) -> Option<u8> {
    xs.iter().try_fold(0u8, |total, &x| total.checked_add(x))
}
