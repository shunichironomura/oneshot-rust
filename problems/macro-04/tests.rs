use crate::judge::{case, check, check_eq};
use crate::user::{char_class, is_small, is_vowel};

// The macros are called by path only, as the statement says.

pub fn run() {
    case("example-1", || {
        check(crate::user::matches_any!(5, 1 | 4..=6), "5 matches 1 | 4..=6")?;
        check(
            !crate::user::matches_any!(Some(7), Some(1..=3) | None),
            "Some(7) does not match Some(1..=3) | None",
        )
    });
    case("example-2", || {
        let i: Option<usize> = crate::user::index_of!(7; 0..=4, 5..=9, _);
        check_eq(i, Some(1))
    });
    case("example-3", || {
        check_eq(is_vowel('E'), true)?;
        check_eq(is_small(Some(12)), false)?;
        check_eq(char_class('7'), Some(1))
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
        // `value` is evaluated exactly once, whether a pattern matches or not.
        let mut n = 0;
        let a = crate::user::matches_any!({ n += 1; 3 }, 1 | 2 | 4..=9);
        let b = crate::user::matches_any!({ n += 10; 8 }, 1 | 2 | 4..=9);
        check_eq((a, b, n), (false, true, 11))?;
        let mut m = 0;
        let i: Option<usize> = crate::user::index_of!({ m += 1; 'z' }; 'a', 'b' | 'c', 'd');
        let j: Option<usize> = crate::user::index_of!({ m += 10; 'd' }; 'a', 'b' | 'c', 'd');
        let k: Option<usize> = crate::user::index_of!({ m += 100; 'a' }; 'a', 'b' | 'c', 'd');
        check_eq((i, j, k, m), (None, Some(2), Some(0), 111))
    });
    case("case-3", || {
        // The first matching pattern wins; `_` matches anything.
        let a: Option<usize> = crate::user::index_of!(5; 0..=3, 4..=9, 5);
        let b: Option<usize> = crate::user::index_of!(5; 5, 0..=9);
        let c: Option<usize> = crate::user::index_of!(42; 1, _, 42);
        let d: Option<usize> = crate::user::index_of!(-3; 0.., ..0, _);
        let e: Option<usize> = crate::user::index_of!(10; 0..=3, 4..=9);
        check_eq((a, b, c, d, e), (Some(1), Some(0), Some(1), Some(1), None))
    });
    case("case-4", || {
        // One pattern, a trailing comma, and a longer list.
        let a: Option<usize> = crate::user::index_of!(3; 3);
        let b: Option<usize> = crate::user::index_of!(3; 4,);
        let c: Option<usize> = crate::user::index_of!('b'; 'a', 'b',);
        let d: Option<usize> = crate::user::index_of!(8u8; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
        let e: Option<usize> = crate::user::index_of!(9u8; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
        check_eq((a, b, c, d, e), (Some(0), None, Some(1), Some(8), Some(9)))
    });
    case("case-5", || {
        // Or-patterns and nested patterns in the list.
        let a: Option<usize> = crate::user::index_of!('e'; 'x' | 'y', 'a' | 'e' | 'i');
        let b: Option<usize> = crate::user::index_of!(Some(2); None, Some(0) | Some(1), Some(_));
        let c: Option<usize> = crate::user::index_of!(None::<u8>; Some(_), None | Some(0));
        let d: Option<usize> = crate::user::index_of!((3, 'b'); (0, _), (_, 'a' | 'b'), _);
        let e: Option<usize> = crate::user::index_of!("plum"; "apple" | "banana", "pear" | "plum");
        let f: Option<usize> = crate::user::index_of!(Some(4); Some(1 | 2), Some(3 | 5));
        check_eq((a, b, c, d, e, f), (Some(1), Some(2), Some(1), Some(1), Some(1), None))
    });
    case("case-6", || {
        for c in ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'] {
            check(is_vowel(c), "an ASCII vowel")?;
        }
        for c in ['y', 'Y', 'b', 'z', 'Q', '1', ' ', 'é', 'À', 'ı'] {
            check(!is_vowel(c), "not an ASCII vowel")?;
        }
        let small = [None, Some(0), Some(9), Some(10), Some(255)].map(is_small);
        check_eq(small, [true, true, true, false, false])
    });
    case("case-7", || {
        let letters = ['a', 'm', 'z', 'A', 'Q', 'Z'].map(char_class);
        let digits = ['0', '5', '9'].map(char_class);
        let spaces = [' ', '\t', '\n'].map(char_class);
        check_eq(letters, [Some(0); 6])?;
        check_eq(digits, [Some(1); 3])?;
        check_eq(spaces, [Some(2); 3])?;
        let others = ['_', '-', '/', ':', '@', '[', '`', '{', '\r', 'é', 'Ω', '٣', '\u{3000}'];
        check_eq(others.map(char_class), [None; 13])
    });
}
