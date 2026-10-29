use crate::judge::{case, check_eq};
use crate::user::{Keep, select};

fn pick(v: &[i32], keep: Keep) -> Vec<i32> {
    select(v, keep).collect()
}

pub fn run() {
    case("example-1", || check_eq(pick(&[1, -3, 4], Keep::Odds), vec![1, -3]));
    case("case-1", || {
        // Negative even numbers and zero are even.
        check_eq(pick(&[-4, -3, 0, 7], Keep::Evens), vec![-4, 0])?;
        check_eq(pick(&[6, 5, 4, 3, 2, 1], Keep::Evens), vec![6, 4, 2])?;
        check_eq(pick(&[i32::MIN, -1, i32::MAX], Keep::Evens), vec![i32::MIN])
    });
    case("case-2", || {
        // Negative odd numbers are odd: -7 % 2 is -1, not 1.
        check_eq(pick(&[-7, -2, -1, 0, 5, 9], Keep::Odds), vec![-7, -1, 5, 9])?;
        check_eq(pick(&[i32::MIN, -1, i32::MAX], Keep::Odds), vec![-1, i32::MAX])?;
        check_eq(pick(&[2, -8, 0], Keep::Odds), Vec::new())
    });
    case("case-3", || {
        // All keeps every element in order, and an empty slice gives nothing.
        let v = vec![3, -1, 3, 0, -6];
        check_eq(pick(&v, Keep::All), vec![3, -1, 3, 0, -6])?;
        check_eq(select(&v, Keep::All).sum::<i32>(), -1)?;
        check_eq(pick(&[], Keep::Evens), Vec::new())?;
        check_eq(pick(&[], Keep::Odds), Vec::new())?;
        check_eq(pick(&[], Keep::All), Vec::new())
    });
}
