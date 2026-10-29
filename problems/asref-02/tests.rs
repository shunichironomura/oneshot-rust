use crate::judge::{case, check_eq};
use crate::user::total_chars;

pub fn run() {
    case("example-1", || {
        check_eq(total_chars(vec![vec!["ab", "c"], vec![]]), 3)?;
        check_eq(total_chars([["a", "b"]; 3]), 6)?;
        check_eq(total_chars(&[vec![String::from("é")]]), 1)?;
        check_eq(total_chars("a b\nc".lines().map(|l| l.split(' '))), 3)
    });
    case("case-1", || {
        check_eq(total_chars(Vec::<Vec<&str>>::new()), 0)?;
        check_eq(total_chars([Vec::<String>::new(), Vec::new()]), 0)
    });
    case("case-2", || {
        let groups = vec![vec![String::from("Rust"), String::new()], vec![String::from("🦀")]];
        check_eq(total_chars(&groups), 5)?;
        check_eq(total_chars(groups), 5)
    });
    case("case-3", || {
        check_eq(total_chars("x,yz;;w".split(';').map(|p| p.split(','))), 4)?;
        // "e" followed by a combining accent: two `char`s.
        check_eq(total_chars([[String::from("e\u{301}"), String::from("日本語")]]), 5)
    });
}
