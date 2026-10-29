use crate::judge::{case, check_eq};
use crate::user::parallel_sum;

pub fn run() {
    case("example-1", || check_eq(parallel_sum(&[1, 2, 3, 4, 5], 2), 15));
    case("case-1", || check_eq(parallel_sum(&[], 3), 0));
    case("case-2", || check_eq(parallel_sum(&[7], 4), 7));
    case("case-3", || {
        let values: Vec<u64> = (1..=40).collect();
        check_eq(parallel_sum(&values, 3), 820)
    });
    case("case-4", || {
        let values = vec![10, 20, 30];
        check_eq(parallel_sum(&values, 1), 60)
    });
}
