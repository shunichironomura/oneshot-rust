use std::cmp::Ordering;

use crate::judge::{case, check, check_eq};

// No `use` for the macros or for `HashMap`: the macros must work by path alone.
type Map<K, V> = std::collections::HashMap<K, V>;

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
    case("sample-1", || {
        let m: Map<&str, i32> = crate::user::hashmap! { "a" => 1, "b" => 2, };
        check_eq(m.len(), 2)?;
        check_eq(m["a"], 1)?;
        check_eq(m["b"], 2)
    });
    case("sample-2", || {
        let x: i32 = crate::user::max_of!(3, 9, 4);
        check_eq(x, 9)
    });
    case("sample-3", || {
        const N: usize = crate::user::count!(a + (b c));
        const Z: usize = crate::user::count!();
        check_eq(N, 3)?;
        check_eq(Z, 0)
    });
    case("hidden-1", || {
        // Empty maps take their types from the context; the last value of a key wins.
        let empty: Map<String, Vec<u8>> = crate::user::hashmap! {};
        check(empty.is_empty(), "hashmap! {} should be empty")?;
        let one: Map<char, u8> = crate::user::hashmap! { 'z' => 26 };
        check_eq(one.len(), 1)?;
        check_eq(one[&'z'], 26)?;
        let m: Map<String, i32> = crate::user::hashmap! {
            String::from("x") => 1,
            "y".to_string() => 10 * 2,
            String::from("x") => 3,
        };
        check_eq(m.len(), 2)?;
        check_eq(m["x"], 3)?;
        check_eq(m["y"], 20)
    });
    case("hidden-2", || {
        // Every key and value is evaluated once, in the order k1, v1, k2, v2, ...
        let mut log: Vec<&str> = Vec::new();
        let m: Map<u8, char> = crate::user::hashmap! {
            { log.push("k1"); 1 } => { log.push("v1"); 'a' },
            { log.push("k2"); 2 } => { log.push("v2"); 'b' },
            { log.push("k3"); 1 } => { log.push("v3"); 'c' }
        };
        check_eq(log, vec!["k1", "v1", "k2", "v2", "k3", "v3"])?;
        check_eq(m.len(), 2)?;
        check_eq(m[&1], 'c')?;
        check_eq(m[&2], 'b')
    });
    case("hidden-3", || {
        // One argument, trailing commas, and names that clash with a macro's own locals.
        let a: i32 = crate::user::max_of!(7);
        let b: i32 = crate::user::max_of!(-5,);
        let c: i32 = crate::user::max_of!(1, 2,);
        let (max, next, map) = (4, 6, 5);
        let d: i32 = crate::user::max_of!(max, next, map);
        check_eq((a, b, c, d), (7, -5, 2, 6))
    });
    case("hidden-4", || {
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
    case("hidden-5", || {
        // On a tie, the leftmost of the greatest values is the result.
        let t: Tagged =
            crate::user::max_of!(Tagged(2, "a"), Tagged(5, "b"), Tagged(1, "c"), Tagged(5, "d"));
        check_eq(t.1, "b")?;
        let t: Tagged = crate::user::max_of!(Tagged(0, "x"), Tagged(0, "y"), Tagged(0, "z"));
        check_eq(t.1, "x")
    });
    case("hidden-6", || {
        // Values that are not Copy, and types that are PartialOrd but not Ord.
        let (pear, plum, apple) = (String::from("pear"), String::from("plum"), String::from("apple"));
        let s: String = crate::user::max_of!(pear, plum, apple);
        check_eq(s.as_str(), "plum")?;
        let f: f64 = crate::user::max_of!(1.5, -2.0, 3.25, 0.5);
        check_eq(f, 3.25)?;
        let w: &str = crate::user::max_of!("b", "c", "a");
        check_eq(w, "c")
    });
    case("hidden-7", || {
        // Groups count as one token tree; punctuation, literals and lifetimes count too.
        const A: usize = crate::user::count!(fn main() { println!("hi"); });
        const B: usize = crate::user::count!(x :: y => 'a "s" 1.5 [1, 2, 3]);
        const C: usize = crate::user::count!(a, b, c);
        const D: usize = crate::user::count!((a, b));
        let array = [0u8; crate::user::count!(x y z)];
        check_eq((A, B, C, D), (4, 8, 5, 1))?;
        check_eq(array.len(), 3)
    });
}
