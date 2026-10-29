use crate::judge::{case, check_eq};
use crate::user::bonus_total;

fn entry(name: &str, score: u32) -> (String, u32) {
    (name.to_string(), score)
}

pub fn run() {
    case("example-1", || check_eq(bonus_total(&[entry("anna", 1), entry("bo", 2)]), 8));
    case("case-1", || {
        // Exactly 3 bytes earns nothing.
        check_eq(bonus_total(&[entry("abc", 10), entry("abcd", 10)]), 25)?;
        check_eq(bonus_total(&[entry("long name", 0), entry("x", 4), entry("four", 6)]), 20)
    });
    case("case-2", || {
        // The length is in bytes, not chars.
        check_eq(bonus_total(&[entry("日本", 1), entry("éa", 1), entry("", 0)]), 7)
    });
    case("case-3", || check_eq(bonus_total(&[]), 0));
}
