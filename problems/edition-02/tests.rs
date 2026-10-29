use crate::judge::{case, check_eq};
use crate::user::score;

pub fn run() {
    case("example-1", || check_eq(score(vec![3, 4].into_boxed_slice()), 9));
    case("case-1", || {
        check_eq(score(Vec::new().into_boxed_slice()), 0)?;
        check_eq(score(vec![0].into_boxed_slice()), 1)?;
        check_eq(score(vec![10, 0, 25, 4, 1].into_boxed_slice()), 45)
    });
}
