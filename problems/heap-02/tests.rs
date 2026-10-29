use crate::judge::{case, check_eq};
use crate::user::k_smallest;

pub fn run() {
    case("example-1", || check_eq(k_smallest(&[5, 1, 4, 1, 3], 3), vec![1, 1, 3]));
    case("case-1", || {
        check_eq(k_smallest(&[4, 2, 9], 0), vec![])?;
        check_eq(k_smallest(&[], 3), vec![])?;
        check_eq(k_smallest(&[], 0), vec![])
    });
    case("case-2", || {
        // k is v.len() or more: everything, sorted.
        let v = [3, -2, 7, -2];
        check_eq(k_smallest(&v, 4), vec![-2, -2, 3, 7])?;
        check_eq(k_smallest(&v, 10), vec![-2, -2, 3, 7])?;
        check_eq(k_smallest(&v, usize::MAX), vec![-2, -2, 3, 7])
    });
    case("case-3", || {
        let v = [i32::MAX, -5, 10, i32::MIN, 0, -5, 7, 6, 8, -1];
        check_eq(k_smallest(&v, 4), vec![i32::MIN, -5, -5, -1])?;
        check_eq(k_smallest(&v, 1), vec![i32::MIN])?;
        check_eq(k_smallest(&[i32::MAX, i32::MAX, 8], 2), vec![8, i32::MAX])
    });
}
