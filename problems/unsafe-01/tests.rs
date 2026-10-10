use crate::judge::{case, check_eq};
use crate::user::push_and_sum;

pub fn run() {
    case("sample-1", || {
        let mut v = vec![1, 2, 3];
        let r = push_and_sum(&mut v, 4);
        check_eq(v, vec![1, 2, 3, 4])?;
        check_eq(r, 11)
    });
    case("hidden-1", || {
        let mut v = Vec::with_capacity(8);
        v.push(5);
        check_eq(push_and_sum(&mut v, -5), 5)
    });
    case("hidden-2", || {
        let mut v = vec![-1];
        check_eq(push_and_sum(&mut v, 10), 8)
    });
}
