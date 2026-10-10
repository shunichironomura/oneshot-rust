use crate::judge::{case, check_eq};
use crate::user::largest;

pub fn run() {
    case("sample-1", || check_eq(*largest(&[3, 7, 2]), 7));
    case("sample-2", || check_eq(*largest(&[1.5, -2.0, 0.25]), 1.5));
    case("sample-3", || {
        let words = vec![String::from("apple"), String::from("pear"), String::from("fig")];
        let w: &String = largest(&words);
        check_eq(w.as_str(), "pear")
    });
    case("hidden-1", || check_eq(*largest(&['a', 'z', 'm']), 'z'));
}
