use crate::judge::{case, check_eq};
use crate::user::push_doubled;

fn apply(mut v: Vec<i32>) -> Vec<i32> {
    push_doubled(&mut v);
    v
}

pub fn run() {
    case("example-1", || check_eq(apply(vec![3, -1, 2]), vec![3, -1, 2, 6, 4]));
    case("case-1", || check_eq(apply(vec![]), vec![]));
    case("case-2", || check_eq(apply(vec![-5, 0]), vec![-5, 0]));
    case("case-3", || check_eq(apply(vec![1, 1]), vec![1, 1, 2, 2]));
}
