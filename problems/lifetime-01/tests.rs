use crate::judge::{case, check, check_eq};
use crate::user::longest_word;

pub fn run() {
    case("example-1", || check_eq(longest_word("a bb ccc"), "ccc"));
    case("example-2", || check_eq(longest_word("xx yy"), "xx"));
    case("case-1", || check_eq(longest_word(""), ""));
    case("case-2", || check_eq(longest_word("  spaced   out  "), "spaced"));
    case("case-3", || {
        let owned = String::from("borrow me please");
        let w = longest_word(&owned);
        check_eq(w, "borrow")?;
        // The result must point into the input (no copies, no static strings).
        let base = owned.as_ptr() as usize;
        let p = w.as_ptr() as usize;
        check(base <= p && p < base + owned.len(), "not a substring of the input")
    });
}
