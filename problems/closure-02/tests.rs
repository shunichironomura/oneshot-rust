use crate::judge::{case, check, check_eq};
use crate::user::{apply_n, call_shared, consume, each_digit, map_pair};

fn seven() -> u32 {
    7
}

pub fn run() {
    case("example-1", || {
        let mut count = 0;
        apply_n(|| count += 2, 3);
        check_eq(count, 6)?;
        let name = String::from("Ferris");
        check_eq(consume(move || name), "Ferris")?;
        check_eq(consume(|| vec![1, 2]), [1, 2])
    });
    case("example-2", || {
        let shout = map_pair(|s: &str| format!("{s}!"), ("a", "b"));
        check_eq(shout, ("a!".to_string(), "b!".to_string()))?;
        check_eq(map_pair(|n: i32| n * n, (3, -4)), (9, 16))?;
        let k = 5;
        check_eq(call_shared(|| 2), 4)?;
        check_eq(call_shared(|| k + 1), 12)
    });
    case("example-3", || {
        let mut digits = Vec::new();
        each_digit(4096, &mut |d| digits.push(d));
        check_eq(digits, [4, 0, 9, 6])
    });
    case("case-1", || {
        // n = 0 never calls `f`, and a `&mut` to an FnMut closure is an FnMut too.
        let mut log = Vec::new();
        apply_n(|| log.push(99), 0);
        check(log.is_empty(), "f must not be called when n is 0")?;
        let mut push_len = || log.push(log.len());
        apply_n(&mut push_len, 2);
        apply_n(&mut push_len, 2);
        check_eq(log, [0, 1, 2, 3])
    });
    case("case-2", || {
        let words = vec![String::from("b"), String::from("a")];
        let sorted = consume(move || {
            let mut words = words;
            words.sort();
            words
        });
        check_eq(sorted, ["a", "b"])?;
        check_eq(consume(|| (1u8, 'x', true)), (1u8, 'x', true))?;
        check_eq(consume(String::new), "")
    });
    case("case-3", || {
        // The closure may update captured state; the first element is mapped first.
        let mut next = 0;
        let numbered = map_pair(
            |name: &str| {
                next += 1;
                format!("{next}. {name}")
            },
            ("Ann", "Bob"),
        );
        check_eq(numbered, ("1. Ann".to_string(), "2. Bob".to_string()))?;
        check_eq(next, 2)?;
        check_eq(map_pair(|n: i32| n % 2 == 0, (4, 7)), (true, false))?;
        check_eq(map_pair(str::len, ("ab", "xyz")), (2, 3))?;
        check_eq(map_pair(|v: Vec<u8>| v.len(), (vec![1], Vec::new())), (1, 0))
    });
    case("case-4", || {
        let word = String::from("closure");
        check_eq(call_shared(|| word.len() as u32), 14)?;
        check_eq(call_shared(seven), 14)?;
        check_eq(word, "closure")
    });
    case("case-5", || {
        // 0 has one digit, and zeros inside a number are digits too.
        let mut digits = Vec::new();
        each_digit(0, &mut |d| digits.push(d));
        check_eq(&digits, &[0])?;
        digits.clear();
        each_digit(100, &mut |d| digits.push(d));
        check_eq(&digits, &[1, 0, 0])?;
        let mut calls = 0;
        each_digit(u64::MAX, &mut |_| calls += 1);
        check_eq(calls, 20)?;
        digits.clear();
        each_digit(u64::MAX, &mut |d| digits.push(d));
        check_eq(digits.iter().fold(0, |n, &d| n * 10 + d), u64::MAX)
    });
    case("case-6", || {
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
