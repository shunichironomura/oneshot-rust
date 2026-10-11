use crate::judge::{case, check, check_eq};
use crate::user::{apply_n, call_shared, consume, map_pair, ops_table};

fn seven() -> u32 {
    7
}

pub fn run() {
    case("sample-1", || {
        let mut count = 0;
        apply_n(|| count += 2, 3);
        check_eq(count, 6)?;
        let name = String::from("Ferris");
        check_eq(consume(move || name), "Ferris")?;
        check_eq(consume(|| vec![1, 2]), [1, 2])
    });
    case("sample-2", || {
        let shout = map_pair(|s: &str| format!("{s}!"), ("a", "b"));
        check_eq(shout, ("a!".to_string(), "b!".to_string()))?;
        check_eq(map_pair(|n: i32| n * n, (3, -4)), (9, 16))?;
        let k = 5;
        check_eq(call_shared(|| 2), 4)?;
        check_eq(call_shared(|| k + 1), 12)
    });
    case("sample-3", || {
        let mut results = Vec::new();
        for (op, f) in ops_table() {
            results.push((op, f(7, 3)));
        }
        check_eq(results, [('+', 10), ('-', 4), ('*', 21)])
    });
    case("hidden-1", || {
        // n = 0 never calls `f`, and a `&mut` to an FnMut closure is an FnMut too.
        let mut log = Vec::new();
        apply_n(|| log.push(99), 0);
        check(log.is_empty(), "f must not be called when n is 0")?;
        let mut push_len = || log.push(log.len());
        apply_n(&mut push_len, 2);
        apply_n(&mut push_len, 2);
        check_eq(log, [0, 1, 2, 3])
    });
    case("hidden-2", || {
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
    case("hidden-3", || {
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
    case("hidden-4", || {
        let word = String::from("closure");
        check_eq(call_shared(|| word.len() as u32), 14)?;
        check_eq(call_shared(seven), 14)?;
        check_eq(word, "closure")
    });
    case("hidden-5", || {
        let table = ops_table();
        let symbols = table.map(|(op, _)| op);
        check_eq(symbols, ['+', '-', '*'])?;
        let (_, sub) = table[1];
        let (_, mul) = table[2];
        check_eq(sub(3, 10), -7i32)?;
        check_eq(mul(3, 4), 12)?;
        check_eq(table.map(|(_, f)| f(-2, 5)), [3, -7, -10])
    });
}
