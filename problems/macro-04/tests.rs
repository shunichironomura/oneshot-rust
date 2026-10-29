use crate::judge::{case, check, check_eq};
use crate::user::{is_small, is_vowel};

// The macro is called by path only, as the statement says.

pub fn run() {
    case("example-1", || {
        check(crate::user::matches_any!(5, 1 | 4..=6), "5 matches 1 | 4..=6")?;
        check(
            !crate::user::matches_any!(Some(7), Some(1..=3) | None),
            "Some(7) does not match Some(1..=3) | None",
        )
    });
    case("example-2", || {
        check_eq(is_vowel('E'), true)?;
        check_eq(is_small(Some(12)), false)
    });
    case("case-1", || {
        // Top-level and nested or-patterns, ranges, tuples and string literals.
        let results = [
            crate::user::matches_any!(1, 1 | 4..=6),
            crate::user::matches_any!(6, 1 | 4..=6),
            !crate::user::matches_any!(3, 1 | 4..=6),
            !crate::user::matches_any!(7, 1 | 4..=6),
            crate::user::matches_any!(Some(3), Some(1..=3) | None),
            crate::user::matches_any!(None::<i32>, Some(1..=3) | None),
            !crate::user::matches_any!(Some(0), Some(1..=3) | None),
            crate::user::matches_any!(Some(2), Some(1 | 2)),
            !crate::user::matches_any!(Some(3), Some(1 | 2)),
            crate::user::matches_any!('x', 'x'),
            !crate::user::matches_any!('y', 'x'),
            crate::user::matches_any!((2, 'q'), (1, 'a' | 'b') | (2, _)),
            crate::user::matches_any!((1, 'b'), (1, 'a' | 'b') | (2, _)),
            !crate::user::matches_any!((1, 'c'), (1, 'a' | 'b') | (2, _)),
            crate::user::matches_any!("hi", "hello" | "hi" | "hey"),
            !crate::user::matches_any!("Hi", "hello" | "hi" | "hey"),
        ];
        let wrong: Vec<usize> = (0..results.len()).filter(|&k| !results[k]).collect();
        check_eq(wrong, Vec::<usize>::new())
    });
    case("case-2", || {
        // `value` is evaluated exactly once, whether the pattern matches or not.
        let mut n = 0;
        let a = crate::user::matches_any!({ n += 1; 3 }, 1 | 2 | 4..=9);
        let b = crate::user::matches_any!({ n += 10; 8 }, 1 | 2 | 4..=9);
        check_eq((a, b, n), (false, true, 11))
    });
    case("case-3", || {
        for c in ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'] {
            check(is_vowel(c), "an ASCII vowel")?;
        }
        for c in ['y', 'Y', 'b', 'z', 'Q', '1', ' ', 'é', 'À', 'ı'] {
            check(!is_vowel(c), "not an ASCII vowel")?;
        }
        let small = [None, Some(0), Some(9), Some(10), Some(255)].map(is_small);
        check_eq(small, [true, true, true, false, false])
    });
}
