use crate::judge::{case, check_eq};
use crate::user::merge;

const COLORS: [&str; 2] = ["red", "green"];

pub fn run() {
    case("example-1", || check_eq(merge(&["a", "b"], "b, c,,d"), vec!["a", "b", "c", "d"]));
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
}
