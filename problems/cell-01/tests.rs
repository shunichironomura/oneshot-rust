use std::rc::Rc;

use crate::judge::{case, check, check_eq};
use crate::user::Lookup;

const NO_KEYS: [&str; 0] = [];

pub fn run() {
    case("example-1", || {
        let lookup = Lookup::new(&[("b", 2), ("a", 1)]);
        check_eq(lookup.get("a"), Some(1))?;
        check_eq(lookup.get("z"), None)?;
        check_eq(lookup.keys(), ["a", "b"])?;
        check_eq(lookup.keys(), ["a", "b"])?;
        check_eq((lookup.hits(), lookup.misses(), lookup.sorts()), (1, 1, 1))
    });
    case("example-2", || {
        let mut lookup = Lookup::new(&[("b", 2), ("a", 1)]);
        check_eq(lookup.get("a"), Some(1))?;
        check_eq(lookup.get("z"), None)?;
        check_eq(lookup.keys(), ["a", "b"])?;
        check_eq(lookup.keys(), ["a", "b"])?;
        lookup.insert("c", 10);
        check_eq(lookup.keys(), ["a", "b", "c"])?;
        check_eq((lookup.hits(), lookup.misses(), lookup.sorts()), (1, 1, 2))
    });
    case("case-1", || {
        // Two shared references to the same table, used in turn, while a slice returned by
        // `keys` is still in use.
        let lookup = Lookup::new(&[("x", 10), ("Y", 20), ("z", 30), ("10", 1), ("9", 2)]);
        let a = &lookup;
        let b = &lookup;
        let first = a.keys();
        let mut found = 0;
        for (i, key) in ["x", "q", "Y", "x", "w", "z", "y"].into_iter().enumerate() {
            let r = if i % 2 == 0 { a } else { b };
            if let Some(value) = r.get(key) {
                found += value;
            }
            if i == 2 {
                check_eq(b.keys(), ["10", "9", "Y", "x", "z"])?;
            }
        }
        check_eq(first, ["10", "9", "Y", "x", "z"])?;
        check_eq(found, 70)?;
        check_eq((a.hits(), b.misses()), (4, 3))?;
        check_eq((b.hits(), a.misses()), (4, 3))?;
        check_eq((a.sorts(), b.sorts()), (1, 1))
    });
    case("case-2", || {
        // The keys are sorted lazily, and get and keys do not affect each other's counters.
        let lookup = Lookup::new(&[("m", 8), ("k", 7)]);
        check_eq(lookup.sorts(), 0)?;
        check_eq(lookup.get("k"), Some(7))?;
        check_eq(lookup.sorts(), 0)?;
        check_eq(lookup.keys(), ["k", "m"])?;
        check_eq((lookup.hits(), lookup.misses(), lookup.sorts()), (1, 0, 1))?;
        check_eq(lookup.get("n"), None)?;
        check_eq(lookup.get("m"), Some(8))?;
        check_eq(lookup.keys(), ["k", "m"])?;
        check_eq((lookup.hits(), lookup.misses(), lookup.sorts()), (2, 1, 1))
    });
    case("case-3", || {
        // An empty list is cached like any other list, and a value of 0 is still a hit.
        let empty = Lookup::new(&[]);
        check_eq(empty.get("a"), None)?;
        check_eq(empty.keys(), NO_KEYS)?;
        check_eq(empty.keys(), NO_KEYS)?;
        check_eq((empty.hits(), empty.misses(), empty.sorts()), (0, 1, 1))?;
        let zero = Lookup::new(&[("zero", 0)]);
        check_eq(zero.get("zero"), Some(0))?;
        check_eq(zero.keys(), ["zero"])?;
        check_eq(zero.keys(), ["zero"])?;
        check_eq(zero.keys(), ["zero"])?;
        check_eq((zero.hits(), zero.misses(), zero.sorts()), (1, 0, 1))
    });
    case("case-4", || {
        // One table shared through an Rc by two closures.
        let shared = Rc::new(Lookup::new(&[("red", 3), ("green", 4), ("blue", 5)]));
        let find = {
            let lookup = Rc::clone(&shared);
            move |key: &str| lookup.get(key)
        };
        let count = {
            let lookup = Rc::clone(&shared);
            move || lookup.keys().len()
        };
        check_eq(find("red"), Some(3))?;
        check_eq(count(), 3)?;
        let keys = shared.keys();
        check_eq(find("pink"), None)?;
        check_eq(find("blue"), Some(5))?;
        check_eq(count(), 3)?;
        check_eq(keys, ["blue", "green", "red"])?;
        check_eq((shared.hits(), shared.misses(), shared.sorts()), (2, 1, 1))
    });
    case("case-5", || {
        // insert replaces values and discards the cached list, even when the keys stay the same;
        // it does not sort anything itself.
        let mut lookup = Lookup::new(&[("b", 1), ("a", 2)]);
        lookup.insert("a", 40);
        check_eq(lookup.sorts(), 0)?;
        check_eq(lookup.keys(), ["a", "b"])?;
        lookup.insert("b", 0);
        check_eq(lookup.sorts(), 1)?;
        check_eq(lookup.keys(), ["a", "b"])?;
        lookup.insert("C", 5);
        check_eq(lookup.sorts(), 2)?;
        check_eq(lookup.keys(), ["C", "a", "b"])?;
        check_eq(lookup.keys(), ["C", "a", "b"])?;
        check_eq(lookup.get("C"), Some(5))?;
        check_eq(lookup.get("b"), Some(0))?;
        check_eq((lookup.hits(), lookup.misses(), lookup.sorts()), (2, 0, 3))?;
        let view = &lookup;
        let keys = view.keys();
        check_eq(view.get("a"), Some(40))?;
        check(keys.len() == 3 && keys[0] == "C", "the slice from keys should still be valid")?;
        check_eq((lookup.hits(), lookup.sorts()), (3, 3))
    });
}
