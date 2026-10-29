use crate::judge::{case, check_eq};
use crate::user::{merge, unique_upper};

const COLORS: [&str; 2] = ["red", "green"];

pub fn run() {
    case("example-1", || check_eq(merge(&["a", "b"], "b, c,,d"), vec!["a", "b", "c", "d"]));
    case("example-2", || {
        check_eq(unique_upper("to be or not to be"), vec!["TO", "BE", "OR", "NOT"])
    });
    case("case-1", || {
        // Nothing to add: the defaults alone, or nothing at all.
        check_eq(merge(&COLORS, " , ,,\t,"), vec!["red", "green"])?;
        check_eq(merge(&[], ""), Vec::<String>::new())
    });
    case("case-2", || {
        // Repeats of a default or of an earlier word are skipped; the comparison is case-sensitive.
        check_eq(
            merge(&COLORS, "blue, red ,blue,Green,\tpink\n, green"),
            vec!["red", "green", "blue", "Green", "pink"],
        )
    });
    case("case-3", || {
        // `extra` may be a short-lived string; the result owns its words.
        let extra = String::from("dark mode,  big font  ,dark mode");
        let got = merge(&["big font"], &extra);
        drop(extra);
        check_eq(got, vec!["big font", "dark mode"])
    });
    case("case-4", || {
        check_eq(unique_upper(""), Vec::<String>::new())?;
        check_eq(unique_upper(" \t\n  "), Vec::<String>::new())
    });
    case("case-5", || {
        // Any run of whitespace separates words; the result owns its words.
        let text = String::from("  b\ta\n\nb   c a ");
        let got = unique_upper(&text);
        drop(text);
        check_eq(got, vec!["B", "A", "C"])
    });
    case("case-6", || {
        // Words are compared before uppercasing, and uppercasing is Unicode-aware.
        check_eq(unique_upper("a A straße Straße a straße"), vec!["A", "A", "STRASSE", "STRASSE"])
    });
}
