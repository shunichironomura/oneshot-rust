use crate::judge::{case, check, check_eq};
use crate::user::longest_word;

pub fn run() {
    case("sample-1", || check_eq(longest_word("a bb ccc"), "ccc"));
    case("sample-2", || check_eq(longest_word("xx yy"), "xx"));
    case("hidden-1", || check_eq(longest_word(""), ""));
    case("hidden-2", || check_eq(longest_word("  spaced   out  "), "spaced"));
    case("hidden-3", || {
        let owned = String::from("borrow me please");
        let w = longest_word(&owned);
        check_eq(w, "borrow")?;
        // The result must point into the input (no copies, no static strings).
        let base = owned.as_ptr() as usize;
        let p = w.as_ptr() as usize;
        check(base <= p && p < base + owned.len(), "not a substring of the input")
    });
}
