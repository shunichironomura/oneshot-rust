use crate::judge::{case, check_eq};
use crate::user::push_and_sum;

pub fn run() {
    case("example-1", || {
        let mut v = vec![1, 2, 3];
        let r = push_and_sum(&mut v, 4);
        check_eq(v, vec![1, 2, 3, 4])?;
        check_eq(r, 11)
    });
    case("case-1", || {
        let mut v = Vec::with_capacity(8);
        v.push(5);
        check_eq(push_and_sum(&mut v, -5), 5)
    });
    case("case-2", || {
        let mut v = vec![-1];
        check_eq(push_and_sum(&mut v, 10), 8)
    });
}
