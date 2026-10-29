use crate::judge::{case, check_eq};
use crate::user::zip_arrays;

/// Implements neither `Copy` nor `Clone`.
#[derive(Debug, PartialEq)]
struct NoClone(u8);

fn strings<const N: usize>(xs: [&str; N]) -> [String; N] {
    xs.map(String::from)
}

pub fn run() {
    case("example-1", || check_eq(zip_arrays([1, 2], ["a", "b"]), [(1, "a"), (2, "b")]));
    case("case-1", || {
        // Neither side can be copied or cloned, and the order must be kept.
        let got = zip_arrays(
            [NoClone(1), NoClone(2), NoClone(3), NoClone(4)],
            strings(["w", "x", "y", "z"]),
        );
        let expected = [
            (NoClone(1), "w".to_string()),
            (NoClone(2), "x".to_string()),
            (NoClone(3), "y".to_string()),
            (NoClone(4), "z".to_string()),
        ];
        check_eq(got, expected)?;
        let got = zip_arrays(strings(["p", "q", "r"]), [NoClone(9), NoClone(8), NoClone(7)]);
        let expected = [
            ("p".to_string(), NoClone(9)),
            ("q".to_string(), NoClone(8)),
            ("r".to_string(), NoClone(7)),
        ];
        check_eq(got, expected)
    });
    case("case-2", || {
        let expected: [(NoClone, String); 0] = [];
        check_eq(zip_arrays::<NoClone, String, 0>([], []), expected)
    });
}
