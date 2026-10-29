use crate::judge::{case, check_eq};
use crate::user::{shout, transpose, zip_arrays};

/// Implements neither `Copy` nor `Clone`.
#[derive(Debug, PartialEq)]
struct NoClone(u8);

fn strings<const N: usize>(xs: [&str; N]) -> [String; N] {
    xs.map(String::from)
}

pub fn run() {
    case("example-1", || {
        check_eq(shout(["ann".to_string(), "bob".to_string()]), ["ANN!", "BOB!"])
    });
    case("example-2", || {
        check_eq(transpose([[1, 2, 3], [4, 5, 6]]), [[1, 4], [2, 5], [3, 6]])
    });
    case("example-3", || check_eq(zip_arrays([1, 2], ["a", "b"]), [(1, "a"), (2, "b")]));
    case("case-1", || {
        // No names at all, an empty name, and digits and punctuation that stay as they are.
        let none: [String; 0] = [];
        let expected: [&str; 0] = [];
        check_eq(shout(none), expected)?;
        check_eq(shout(strings(["Hi 2u", "", "R2-D2"])), ["HI 2U!", "!", "R2-D2!"])
    });
    case("case-2", || {
        // Unicode uppercasing: "ß" becomes "SS", so the name gets longer.
        check_eq(shout(strings(["straße", "élan", "ωμέγα"])), ["STRASSE!", "ÉLAN!", "ΩΜΈΓΑ!"])
    });
    case("case-3", || {
        check_eq(transpose([[7]]), [[7]])?;
        check_eq(transpose([[1, -2, 3, -4]]), [[1], [-2], [3], [-4]])?;
        check_eq(transpose([[5], [6], [7], [8]]), [[5, 6, 7, 8]])?;
        check_eq(
            transpose([[1, 2, 3], [4, 5, 6], [7, 8, 9]]),
            [[1, 4, 7], [2, 5, 8], [3, 6, 9]],
        )
    });
    case("case-4", || {
        // Zero rows give C empty rows; zero columns give no rows at all.
        let three_empty_rows: [[i32; 0]; 3] = [[], [], []];
        check_eq(transpose::<0, 3>([]), three_empty_rows)?;
        let no_rows: [[i32; 2]; 0] = [];
        check_eq(transpose::<2, 0>([[], []]), no_rows)
    });
    case("case-5", || {
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
    case("case-6", || {
        let expected: [(NoClone, String); 0] = [];
        check_eq(zip_arrays::<NoClone, String, 0>([], []), expected)
    });
}
