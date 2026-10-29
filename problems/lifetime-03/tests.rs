use crate::judge::{case, check_eq};
use crate::user::matching_lines;

// The pattern is built in a block and dropped before the lines are read.
fn search<'t>(text: &'t str, words: &[&str]) -> impl Iterator<Item = &'t str> {
    let pattern = words.join(" ");
    matching_lines(text, &pattern)
}

pub fn run() {
    case("example-1", || {
        let text = "Rust is fast\nPython is easy\nI trust rustc";
        let found: Vec<&str> = search(text, &["rust"]).collect();
        check_eq(found, vec!["Rust is fast", "I trust rustc"])
    });
    case("case-1", || {
        let text = String::from("alpha\nbeta\ngamma");
        let mut lines = {
            let pattern = String::from("A");
            matching_lines(&text, &pattern)
        };
        check_eq(lines.next(), Some("alpha"))?;
        check_eq(lines.next(), Some("beta"))?;
        check_eq(lines.next(), Some("gamma"))?;
        check_eq(lines.next(), None)
    });
    case("case-2", || {
        let found: Vec<&str> = search("", &["x"]).collect();
        check_eq(found, Vec::<&str>::new())
    });
    case("case-3", || {
        let text = "one two\ntwo three\nthree one";
        let found: Vec<&str> = search(text, &["two", "three"]).collect();
        check_eq(found, vec!["two three"])
    });
    case("case-4", || {
        let text = "Hello\n\nWorld";
        let found: Vec<&str> = search(text, &[]).collect();
        check_eq(found, vec!["Hello", "", "World"])
    });
}
