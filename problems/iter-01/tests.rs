use crate::judge::{case, check_eq};
use crate::user::sum_even_squares;

pub fn run() {
    case("sample-1", || check_eq(sum_even_squares(&[1, 2, 3, 4]), 20));
    case("sample-2", || check_eq(sum_even_squares(&[]), 0));
    case("hidden-1", || check_eq(sum_even_squares(&[-2, -3, 5]), 4));
    case("hidden-2", || check_eq(sum_even_squares(&[1, 3, 5, 7]), 0));
    case("hidden-3", || check_eq(sum_even_squares(&[1_000_000, 2]), 1_000_000_000_004));
}
