use crate::judge::{case, check_eq};
use crate::user::each_digit;

pub fn run() {
    case("example-1", || {
        let mut digits = Vec::new();
        each_digit(4096, &mut |d| digits.push(d));
        check_eq(digits, [4, 0, 9, 6])
    });
    case("case-1", || {
        // 0 has one digit, and zeros inside a number are digits too.
        let mut digits = Vec::new();
        each_digit(0, &mut |d| digits.push(d));
        check_eq(&digits, &[0])?;
        digits.clear();
        each_digit(100, &mut |d| digits.push(d));
        check_eq(&digits, &[1, 0, 0])
    });
    case("case-2", || {
        let mut calls = 0;
        each_digit(u64::MAX, &mut |_| calls += 1);
        check_eq(calls, 20)?;
        let mut digits = Vec::new();
        each_digit(u64::MAX, &mut |d| digits.push(d));
        check_eq(digits.iter().fold(0, |n, &d| n * 10 + d), u64::MAX)
    });
    case("case-3", || {
        // The same closure can be passed again, and what it captured is usable afterwards.
        let mut sum = 0;
        let mut max = 0;
        let mut add = |d: u64| {
            sum += d;
            max = max.max(d);
        };
        each_digit(1234, &mut add);
        each_digit(907, &mut add);
        each_digit(5, &mut add);
        check_eq((sum, max), (31, 9))
    });
}
