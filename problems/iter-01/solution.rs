pub fn sum_even_squares(v: &[i64]) -> i64 {
    v.iter().filter(|&&x| x % 2 == 0).map(|&x| x * x).sum()
}
