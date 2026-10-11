use crate::judge::{case, check, check_eq};
use crate::user::{Validator, banning};

fn no_digits(s: &str) -> bool {
    !s.chars().any(|c| c.is_ascii_digit())
}

pub fn run() {
    case("sample-1", || {
        // Rules that borrow local variables.
        let min = 3;
        let reserved = vec![String::from("admin"), String::from("root")];
        let mut v = Validator::new();
        v.add(|s| s.len() >= min);
        v.add(|s| !reserved.iter().any(|r| r == s));
        check(v.check("alice"), "\"alice\" passes")?;
        check(!v.check("bo"), "\"bo\" is too short")?;
        check(!v.check("root"), "\"root\" is reserved")
    });
    case("sample-2", || {
        // A validator made by `banning`, plus a rule that is already boxed.
        let words = vec![String::from("spam"), String::from("ad")];
        let mut v = banning(&words);
        let no_space: Box<dyn Fn(&str) -> bool> = Box::new(|s| !s.contains(' '));
        v.add_boxed(no_space);
        check_eq(v.passing(&["hello", "spammer", "bad", "good day"]), vec!["hello"])
    });
    case("sample-3", || {
        // The result of `passing` is used after the validator is gone.
        let items = ["kiwi", "banana", "fig"];
        let max = 4;
        let mut v = Validator::new();
        v.add(|s| s.len() <= max);
        let short = v.passing(&items);
        drop(v);
        check_eq(short, vec!["kiwi", "fig"])
    });
    case("hidden-1", || {
        // Without rules, everything passes; banning no words bans nothing.
        let empty = Validator::new();
        check(empty.check(""), "no rules: \"\" passes")?;
        check_eq(empty.passing(&["", "x", "a b"]), vec!["", "x", "a b"])?;
        let none: Vec<String> = Vec::new();
        let v = banning(&none);
        check(v.check(""), "nothing banned: \"\" passes")?;
        check_eq(v.passing(&["spam", ""]), vec!["spam", ""])
    });
    case("hidden-2", || {
        // Banned words match anywhere in the string, and case matters. More rules can be added.
        let words = vec![String::from("ab"), String::from("Z")];
        let min = 2;
        let mut v = banning(&words);
        v.add(|s| s.len() >= min);
        check_eq(v.passing(&["cab", "AB", "zz", "aZ", "x", "abc", "Ab"]), vec!["AB", "zz", "Ab"])?;
        check(!v.check("Zebra"), "\"Zebra\" contains \"Z\"")
    });
    case("hidden-3", || {
        // Boxed rules may borrow too, and ordinary functions are rules as well.
        let suffix = String::from(".rs");
        let fixed: Vec<Box<dyn Fn(&str) -> bool>> = vec![
            Box::new(|s: &str| !s.is_empty()),
            Box::new(|s: &str| !s.starts_with('.')),
        ];
        let mut v = Validator::new();
        v.add_boxed(Box::new(|s: &str| s.ends_with(suffix.as_str())));
        for rule in fixed {
            v.add_boxed(rule);
        }
        v.add(no_digits);
        let files = ["main.rs", ".rs", "a1.rs", "notes.txt", "", "lib.rs"];
        check_eq(v.passing(&files), vec!["main.rs", "lib.rs"])
    });
    case("hidden-4", || {
        // The strings that are checked may be created after the validator.
        let min = 5;
        let mut v = Validator::new();
        v.add(|s| s.len() >= min);
        let name = String::from("ferris");
        check(v.check(&name), "\"ferris\" passes")?;
        check(!v.check(&"ab".repeat(2)), "\"abab\" is too short")?;
        let line = String::from("crab ferris rustacean");
        let words: Vec<&str> = line.split(' ').collect();
        check_eq(v.passing(&words), vec!["ferris", "rustacean"])
    });
    case("hidden-5", || {
        // `passing` returns the strings themselves, not borrows of the slice that holds them.
        let text = String::from("one two three four");
        let banned = vec![String::from("o")];
        let v = banning(&banned);
        let kept = v.passing(&text.split(' ').collect::<Vec<_>>());
        drop(v);
        check_eq(kept, vec!["three"])
    });
    case("hidden-6", || {
        // The result of `passing` outlives the validator and the variables its rules borrow.
        let items = ["pear", "plum", "apple", "peach"];
        let kept = {
            let first = 'p';
            let max = 4;
            let mut v = Validator::new();
            v.add(|s| s.starts_with(first));
            v.add(|s| s.len() <= max);
            v.passing(&items)
        };
        check_eq(kept, vec!["pear", "plum"])
    });
}
