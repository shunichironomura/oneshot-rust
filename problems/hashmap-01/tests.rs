use crate::judge::{case, check_eq};
use crate::user::word_count;

fn expect(xs: &[(&str, usize)]) -> Vec<(String, usize)> {
    xs.iter().map(|&(w, n)| (w.to_string(), n)).collect()
}

pub fn run() {
    case("example-1", || check_eq(word_count("b a B c a b"), expect(&[("b", 3), ("a", 2), ("c", 1)])));
    case("case-1", || check_eq(word_count(""), expect(&[])));
    case("case-2", || check_eq(word_count("  Rust  rust\nRUST "), expect(&[("rust", 3)])));
    case("case-3", || {
        check_eq(word_count("z y x"), expect(&[("x", 1), ("y", 1), ("z", 1)]))
    });
}
