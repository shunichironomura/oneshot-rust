use crate::judge::{case, check_eq};
use crate::user::capitalize_words;

pub fn run() {
    case("sample-1", || check_eq(capitalize_words("hello world"), "Hello World"));
    case("sample-2", || check_eq(capitalize_words("élan vital"), "Élan Vital"));
    case("hidden-1", || check_eq(capitalize_words(""), ""));
    case("hidden-2", || {
        check_eq(capitalize_words("  two\tspaces \n here "), "  Two\tSpaces \n Here ")
    });
    case("hidden-3", || check_eq(capitalize_words("rUST is 1st"), "RUST Is 1st"));
    case("hidden-4", || check_eq(capitalize_words("ßtraße ñu"), "SStraße Ñu"));
    case("hidden-5", || check_eq(capitalize_words("日本 語"), "日本 語"));
}
