use crate::judge::{case, check_eq};
use crate::user::{all_unique, first_repeat};

pub fn run() {
    case("example-1", || {
        check_eq(first_repeat(&["a", "b", "c", "b", "a"]), Some("b"))?;
        check_eq(first_repeat(&["a", "b", "c"]), None)
    });
    case("example-2", || {
        check_eq(all_unique(&["a", "b", "c"]), true)?;
        check_eq(all_unique(&["a", "b", "a"]), false)
    });
    case("case-1", || {
        check_eq(first_repeat(&["x", "x"]), Some("x"))?;
        check_eq(first_repeat(&["b", "A", "a", "c", "c", "b"]), Some("c"))
    });
    case("case-2", || {
        // An empty list, and words that differ only in case.
        check_eq(first_repeat(&[]), None)?;
        check_eq(first_repeat(&["a", "A", "b"]), None)
    });
    case("case-3", || {
        // The result borrows from the original strings, not from the slice.
        let text = String::from("to be or not to be");
        let found = {
            let words: Vec<&str> = text.split(' ').collect();
            first_repeat(&words)
        };
        check_eq(found, Some("to"))
    });
    case("case-4", || {
        check_eq(all_unique(&[]), true)?;
        check_eq(all_unique(&["solo"]), true)?;
        check_eq(all_unique(&["a", "A", "é", "e"]), true)
    });
    case("case-5", || {
        // The repeat may be anywhere, including the very end.
        check_eq(all_unique(&["z", "z"]), false)?;
        check_eq(all_unique(&["p", "q", "r", "s", "p"]), false)
    });
}
