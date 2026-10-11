use crate::judge::{case, check_eq};
use crate::user::fill_squares;

fn squares(n: u64) -> Vec<u64> {
    (0..n).map(|i| i * i).collect()
}

pub fn run() {
    case("sample-1", || {
        let mut v = [7; 5];
        fill_squares(&mut v, 2);
        check_eq(v, [0, 1, 4, 9, 16])
    });
    case("hidden-1", || {
        // An empty slice: there is nothing to write, and nothing may panic.
        let mut v: [u64; 0] = [];
        fill_squares(&mut v, 2);
        check_eq(v, [])
    });
    case("hidden-2", || {
        // More workers than elements.
        let mut v = [5, 5];
        fill_squares(&mut v, 3);
        check_eq(v, [0, 1])
    });
    case("hidden-3", || {
        // A single worker; every old value is overwritten.
        let mut v = vec![u64::MAX; 6];
        fill_squares(&mut v, 1);
        check_eq(v, squares(6))
    });
    case("hidden-4", || {
        // The length is not a multiple of the number of workers.
        let mut v = vec![u64::MAX; 23];
        fill_squares(&mut v, 3);
        check_eq(v, squares(23))
    });
    case("hidden-5", || {
        // Indices count from the start of `out`, and the rest of `v` stays as it was.
        let mut v = vec![9; 10];
        fill_squares(&mut v[3..8], 2);
        check_eq(v, vec![9, 9, 9, 0, 1, 4, 9, 16, 9, 9])
    });
}
