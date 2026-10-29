use crate::judge::{case, check_eq};
use crate::user::fan_in;

pub fn run() {
    case("example-1", || check_eq(fan_in(vec![vec![3, 1], vec![2]]), vec![1, 2, 3]));
    case("case-1", || check_eq(fan_in(Vec::new()), vec![]));
    case("case-2", || check_eq(fan_in(vec![vec![9, 0, 9], vec![]]), vec![0, 9, 9]));
}
