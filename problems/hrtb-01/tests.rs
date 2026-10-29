use crate::judge::{case, check_eq};
use crate::user::map_lines;

fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

pub fn run() {
    case("example-1", || check_eq(map_lines("  a \n b  ", |s| s.trim()), vec!["a", "b"]));
    case("example-2", || {
        check_eq(map_lines("hello world\nfoo bar", first_word), vec!["hello", "foo"])
    });
    case("case-1", || check_eq(map_lines("", |s| s), Vec::<String>::new()));
    case("case-2", || {
        let prefix = String::from("# ");
        let got = map_lines("# x\r\ny\r\n# z", |s| {
            s.strip_prefix(prefix.as_str()).unwrap_or(s)
        });
        check_eq(got, vec!["x", "y", "z"])
    });
    case("case-3", || {
        let text = String::from("one two\n\nthree");
        let got = map_lines(&text, first_word);
        drop(text);
        check_eq(got, vec!["one", "", "three"])
    });
}
