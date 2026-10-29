use crate::judge::{case, check_eq};
use crate::user::capitalize_words;

pub fn run() {
    case("example-1", || check_eq(capitalize_words("hello world"), "Hello World"));
    case("example-2", || check_eq(capitalize_words("élan vital"), "Élan Vital"));
    case("case-1", || check_eq(capitalize_words(""), ""));
    case("case-2", || {
        check_eq(capitalize_words("  two\tspaces \n here "), "  Two\tSpaces \n Here ")
    });
    case("case-3", || check_eq(capitalize_words("rUST is 1st"), "RUST Is 1st"));
    case("case-4", || check_eq(capitalize_words("ßtraße ñu"), "SStraße Ñu"));
    case("case-5", || check_eq(capitalize_words("日本 語"), "日本 語"));
}
