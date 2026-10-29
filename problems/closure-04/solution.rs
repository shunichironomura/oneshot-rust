pub fn each_digit(n: u64, f: &mut impl FnMut(u64)) {
    if n >= 10 {
        each_digit(n / 10, f);
    }
    f(n % 10);
}
