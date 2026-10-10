use crate::judge::{case, check_eq};
use crate::user::greet_all;

fn names(xs: &[&str]) -> Vec<String> {
    xs.iter().map(|s| s.to_string()).collect()
}

pub fn run() {
    case("sample-1", || {
        check_eq(greet_all(names(&["Ferris"])), (names(&["Hello, Ferris!"]), 1))
    });
    case("hidden-1", || check_eq(greet_all(Vec::new()), (Vec::new(), 0)));
    case("hidden-2", || {
        check_eq(
            greet_all(names(&["a", "b", "c"])),
            (names(&["Hello, a!", "Hello, b!", "Hello, c!"]), 3),
        )
    });
}
