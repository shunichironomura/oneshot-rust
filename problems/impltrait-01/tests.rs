use crate::judge::{case, check_eq};
use crate::user::{Keep, describe, op_for, select};

fn pick(v: &[i32], keep: Keep) -> Vec<i32> {
    select(v, keep).collect()
}

pub fn run() {
    case("sample-1", || check_eq(pick(&[1, -3, 4], Keep::Odds), vec![1, -3]));
    case("sample-2", || {
        check_eq(describe(-5).to_string(), "minus 5".to_string())?;
        check_eq(describe(12).to_string(), "12".to_string())
    });
    case("sample-3", || check_eq(op_for('-', 2)(7, 3), 8));
    case("hidden-1", || {
        // Negative even numbers and zero are even.
        check_eq(pick(&[-4, -3, 0, 7], Keep::Evens), vec![-4, 0])?;
        check_eq(pick(&[6, 5, 4, 3, 2, 1], Keep::Evens), vec![6, 4, 2])?;
        check_eq(pick(&[i32::MIN, -1, i32::MAX], Keep::Evens), vec![i32::MIN])
    });
    case("hidden-2", || {
        // Negative odd numbers are odd: -7 % 2 is -1, not 1.
        check_eq(pick(&[-7, -2, -1, 0, 5, 9], Keep::Odds), vec![-7, -1, 5, 9])?;
        check_eq(pick(&[i32::MIN, -1, i32::MAX], Keep::Odds), vec![-1, i32::MAX])?;
        check_eq(pick(&[2, -8, 0], Keep::Odds), Vec::new())
    });
    case("hidden-3", || {
        // All keeps every element in order, and an empty slice gives nothing.
        let v = vec![3, -1, 3, 0, -6];
        check_eq(pick(&v, Keep::All), vec![3, -1, 3, 0, -6])?;
        check_eq(select(&v, Keep::All).sum::<i32>(), -1)?;
        check_eq(pick(&[], Keep::Evens), Vec::new())?;
        check_eq(pick(&[], Keep::Odds), Vec::new())?;
        check_eq(pick(&[], Keep::All), Vec::new())
    });
    case("hidden-4", || {
        check_eq(describe(0).to_string(), "0".to_string())?;
        check_eq(describe(-1).to_string(), "minus 1".to_string())?;
        check_eq(describe(i32::MAX).to_string(), "2147483647".to_string())
    });
    case("hidden-5", || {
        // The absolute value of i32::MIN does not fit in an i32.
        check_eq(describe(i32::MIN).to_string(), "minus 2147483648".to_string())?;
        check_eq(format!("[{}]", describe(-2147483647)), "[minus 2147483647]".to_string())
    });
    case("hidden-6", || {
        let add = op_for('+', 10);
        check_eq((add(2, 3), add(-4, 1)), (50, -30))?;
        check_eq(op_for('*', 4)(2, 3), 24)?;
        check_eq(op_for('-', 1)(3, 7), -4)?;
        // Any other symbol ignores b.
        check_eq(op_for('?', 3)(5, 100), 15)?;
        check_eq(op_for('/', 2)(9, 3), 18)
    });
    case("hidden-7", || {
        // A negative or zero scale, and several functions kept side by side.
        check_eq(op_for('+', -1)(4, 5), -9)?;
        check_eq(op_for('-', -3)(1, 4), 9)?;
        check_eq(op_for('*', 0)(6, 7), 0)?;
        let ops: Vec<_> = ['+', '-', '*', 'x'].into_iter().map(|c| op_for(c, 3)).collect();
        let got: Vec<i32> = ops.iter().map(|f| f(5, 2)).collect();
        check_eq(got, vec![21, 9, 30, 15])
    });
}
