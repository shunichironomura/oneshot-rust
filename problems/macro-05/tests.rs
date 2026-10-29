use std::cmp::Ordering;

use crate::judge::{case, check_eq};

// No `use` for the macro: it must work by path alone.

/// Ordered by `.0` only, so equal values can still be told apart by `.1`. Not `Copy` or `Clone`.
struct Tagged(i32, &'static str);

impl PartialEq for Tagged {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Tagged {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

pub fn run() {
    case("example-1", || {
        let x: i32 = crate::user::max_of!(3, 9, 4);
        check_eq(x, 9)
    });
    case("case-1", || {
        // One argument, trailing commas, and names that clash with a macro's own locals.
        let a: i32 = crate::user::max_of!(7);
        let b: i32 = crate::user::max_of!(-5,);
        let c: i32 = crate::user::max_of!(1, 2,);
        let (max, next, first, rest) = (4, 6, 5, 3);
        let d: i32 = crate::user::max_of!(max, next, first, rest);
        check_eq((a, b, c, d), (7, -5, 2, 6))
    });
    case("case-2", || {
        // Every argument is evaluated once, from left to right.
        let mut log: Vec<i32> = Vec::new();
        let x: i32 = crate::user::max_of!(
            { log.push(1); 5 },
            { log.push(2); 8 },
            { log.push(3); 2 },
            { log.push(4); 9 },
        );
        check_eq(x, 9)?;
        check_eq(log, vec![1, 2, 3, 4])
    });
    case("case-3", || {
        // On a tie, the leftmost of the greatest values is the result.
        let t: Tagged =
            crate::user::max_of!(Tagged(2, "a"), Tagged(5, "b"), Tagged(1, "c"), Tagged(5, "d"));
        check_eq(t.1, "b")?;
        let t: Tagged = crate::user::max_of!(Tagged(0, "x"), Tagged(0, "y"), Tagged(0, "z"));
        check_eq(t.1, "x")
    });
    case("case-4", || {
        // Values that are not Copy, and types that are PartialOrd but not Ord.
        let (pear, plum, apple) = (String::from("pear"), String::from("plum"), String::from("apple"));
        let s: String = crate::user::max_of!(pear, plum, apple);
        check_eq(s.as_str(), "plum")?;
        let f: f64 = crate::user::max_of!(1.5, -2.0, 3.25, 0.5);
        check_eq(f, 3.25)?;
        let w: &str = crate::user::max_of!("b", "c", "a");
        check_eq(w, "c")
    });
}
