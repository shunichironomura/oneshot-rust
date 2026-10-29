use crate::judge::{case, check_eq};
use crate::user::unique_upper;

pub fn run() {
    case("example-1", || {
        check_eq(unique_upper("to be or not to be"), vec!["TO", "BE", "OR", "NOT"])
    });
    case("case-1", || {
        check_eq(unique_upper(""), Vec::<String>::new())?;
        check_eq(unique_upper(" \t\n  "), Vec::<String>::new())
    });
    case("case-2", || {
        // Any run of whitespace separates words; the result owns its words.
        let text = String::from("  b\ta\n\nb   c a ");
        let got = unique_upper(&text);
        drop(text);
        check_eq(got, vec!["B", "A", "C"])
    });
    case("case-3", || {
        // Words are compared before uppercasing, and uppercasing is Unicode-aware.
        check_eq(unique_upper("a A straße Straße a straße"), vec!["A", "A", "STRASSE", "STRASSE"])
    });
}
