use crate::judge::{case, check_eq};
use crate::user::last_long;

pub fn run() {
    case("example-1", || {
        check_eq(last_long(&["tea", "coffee", "milk", "juice"], 5), Some(3))?;
        check_eq(last_long(&["abc", "日本"], 3), Some(0))
    });
    case("case-1", || {
        // The index of the last match, counted from the front; characters, not bytes.
        check_eq(last_long(&["long", "x", "longer", "y"], 4), Some(2))?;
        check_eq(last_long(&["ab", "abcd", "a", "b", "c"], 2), Some(1))?;
        check_eq(last_long(&["日本語", "abcd", "日本"], 3), Some(1))?;
        check_eq(last_long(&["abc", "🦀🦀", "é"], 2), Some(1))?;
        check_eq(last_long(&["a", "", "b"], 0), Some(2))?;
        check_eq(last_long(&["solo"], 4), Some(0))?;
        check_eq(last_long(&["solo"], 5), None)?;
        check_eq(last_long(&["a", "bb", "日本"], 3), None)?;
        check_eq(last_long(&[], 0), None)
    });
}
