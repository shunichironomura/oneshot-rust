use std::rc::Rc;

use crate::judge::{case, check_eq};
use crate::user::Lookup;

pub fn run() {
    case("sample-1", || {
        let lookup = Lookup::new(&[("a", 1), ("b", 2)]);
        check_eq(lookup.get("a"), Some(1))?;
        check_eq(lookup.get("z"), None)?;
        check_eq(lookup.total(), 3)?;
        check_eq(lookup.total(), 3)?;
        check_eq((lookup.hits(), lookup.misses(), lookup.computations()), (1, 1, 1))
    });
    case("sample-2", || {
        let mut lookup = Lookup::new(&[("a", 1), ("b", 2)]);
        check_eq(lookup.get("a"), Some(1))?;
        check_eq(lookup.get("z"), None)?;
        check_eq(lookup.total(), 3)?;
        check_eq(lookup.total(), 3)?;
        lookup.insert("c", 10);
        check_eq(lookup.total(), 13)?;
        check_eq((lookup.hits(), lookup.misses(), lookup.computations()), (1, 1, 2))
    });
    case("hidden-1", || {
        // Two shared references to the same table, used in turn.
        let lookup = Lookup::new(&[("x", 10), ("y", 20), ("z", 30)]);
        let a = &lookup;
        let b = &lookup;
        let mut found = 0;
        for (i, key) in ["x", "q", "y", "x", "w", "z", "v"].into_iter().enumerate() {
            let r = if i % 2 == 0 { a } else { b };
            if let Some(value) = r.get(key) {
                found += value;
            }
            if i == 2 {
                check_eq(b.total(), 60)?;
            }
        }
        check_eq(found, 70)?;
        check_eq((a.hits(), b.misses()), (4, 3))?;
        check_eq((b.hits(), a.misses()), (4, 3))?;
        check_eq(a.total(), 60)?;
        check_eq((a.computations(), b.computations()), (1, 1))
    });
    case("hidden-2", || {
        // The sum is computed lazily, and get and total do not affect each other's counters.
        let lookup = Lookup::new(&[("k", 7), ("m", 8)]);
        check_eq(lookup.computations(), 0)?;
        check_eq(lookup.get("k"), Some(7))?;
        check_eq(lookup.computations(), 0)?;
        check_eq(lookup.total(), 15)?;
        check_eq((lookup.hits(), lookup.misses(), lookup.computations()), (1, 0, 1))?;
        check_eq(lookup.get("n"), None)?;
        check_eq(lookup.get("m"), Some(8))?;
        check_eq(lookup.total(), 15)?;
        check_eq((lookup.hits(), lookup.misses(), lookup.computations()), (2, 1, 1))
    });
    case("hidden-3", || {
        // A sum of 0 is cached like any other sum, and a value of 0 is still a hit.
        let empty = Lookup::new(&[]);
        check_eq(empty.get("a"), None)?;
        check_eq(empty.total(), 0)?;
        check_eq(empty.total(), 0)?;
        check_eq((empty.hits(), empty.misses(), empty.computations()), (0, 1, 1))?;
        let zero = Lookup::new(&[("zero", 0)]);
        check_eq(zero.get("zero"), Some(0))?;
        check_eq(zero.total(), 0)?;
        check_eq(zero.total(), 0)?;
        check_eq(zero.total(), 0)?;
        check_eq((zero.hits(), zero.misses(), zero.computations()), (1, 0, 1))
    });
    case("hidden-4", || {
        // One table shared through an Rc by two closures.
        let shared = Rc::new(Lookup::new(&[("red", 3), ("green", 4), ("blue", 5)]));
        let find = {
            let lookup = Rc::clone(&shared);
            move |key: &str| lookup.get(key)
        };
        let sum = {
            let lookup = Rc::clone(&shared);
            move || lookup.total()
        };
        check_eq(find("red"), Some(3))?;
        check_eq(sum(), 12)?;
        check_eq(find("pink"), None)?;
        check_eq(find("blue"), Some(5))?;
        check_eq(sum(), 12)?;
        check_eq((shared.hits(), shared.misses(), shared.computations()), (2, 1, 1))
    });
    case("hidden-5", || {
        // insert replaces values and discards the cached sum; it does not compute anything itself.
        let mut lookup = Lookup::new(&[("a", 1), ("b", 2)]);
        lookup.insert("a", 40);
        check_eq(lookup.computations(), 0)?;
        check_eq(lookup.total(), 42)?;
        lookup.insert("b", 0);
        check_eq(lookup.computations(), 1)?;
        check_eq(lookup.total(), 40)?;
        lookup.insert("c", 5);
        check_eq(lookup.computations(), 2)?;
        check_eq(lookup.total(), 45)?;
        check_eq(lookup.total(), 45)?;
        check_eq(lookup.get("c"), Some(5))?;
        check_eq(lookup.get("b"), Some(0))?;
        check_eq((lookup.hits(), lookup.misses(), lookup.computations()), (2, 0, 3))?;
        let view = &lookup;
        check_eq((view.get("a"), view.total()), (Some(40), 45))?;
        check_eq((lookup.hits(), lookup.computations()), (3, 3))
    });
}
