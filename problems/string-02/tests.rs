use crate::judge::{case, check_eq};
use crate::user::join_path;

fn s(text: &str) -> String {
    text.to_string()
}

pub fn run() {
    case("example-1", || check_eq(join_path(s("usr"), s("bin")), "usr/bin"));
    case("case-1", || {
        // An empty dir returns file unchanged.
        check_eq(join_path(s(""), s("a.txt")), "a.txt")?;
        check_eq(join_path(s(""), s("")), "")
    });
    case("case-2", || {
        // An empty file still gets the slash.
        check_eq(join_path(s("usr"), s("")), "usr/")
    });
    case("case-3", || {
        check_eq(join_path(s("/home/ferris"), s("src/main.rs")), "/home/ferris/src/main.rs")
    });
}
