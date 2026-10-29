use crate::judge::{case, check_eq};
use crate::user::map_pair;

pub fn run() {
    case("example-1", || {
        let shout = map_pair(|s: &str| format!("{s}!"), ("a", "b"));
        check_eq(shout, ("a!".to_string(), "b!".to_string()))?;
        check_eq(map_pair(|n: i32| n * n, (3, -4)), (9, 16))
    });
    case("case-1", || {
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
        check_eq(next, 2)
    });
    case("case-2", || {
        check_eq(map_pair(|n: i32| n % 2 == 0, (4, 7)), (true, false))?;
        check_eq(map_pair(str::len, ("ab", "xyz")), (2, 3))?;
        check_eq(map_pair(|v: Vec<u8>| v.len(), (vec![1], Vec::new())), (1, 0))
    });
    case("case-3", || {
        // A `&mut` to an FnMut closure is an FnMut too, so the same closure can be passed again.
        let mut seen = Vec::new();
        let mut record = |c: char| {
            seen.push(c);
            seen.len()
        };
        check_eq(map_pair(&mut record, ('x', 'y')), (1, 2))?;
        check_eq(map_pair(&mut record, ('z', 'x')), (3, 4))?;
        check_eq(seen, ['x', 'y', 'z', 'x'])
    });
}
