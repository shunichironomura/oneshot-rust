use crate::judge::{case, check_eq};
use crate::user::letters_used;

/// An array of 26 empty buckets, with the given buckets filled in.
fn buckets(filled: &[(usize, &[&str])]) -> [Vec<String>; 26] {
    let mut buckets: [Vec<String>; 26] = [const { Vec::new() }; 26];
    for &(i, words) in filled {
        buckets[i] = words.iter().map(|w| w.to_string()).collect();
    }
    buckets
}

pub fn run() {
    case("example-1", || {
        check_eq(letters_used(&buckets(&[(0, &["Al"]), (1, &["bob", "beth"])])), "ab")
    });
    case("example-2", || {
        check_eq(letters_used(&buckets(&[(24, &["Yak"]), (25, &["zoo", "zebra"])])), "yz")
    });
    case("case-1", || check_eq(letters_used(&buckets(&[])), ""));
    case("case-2", || {
        // Only the first or only the last bucket.
        check_eq(letters_used(&buckets(&[(0, &["a"])])), "a")?;
        check_eq(letters_used(&buckets(&[(25, &["z"])])), "z")
    });
    case("case-3", || {
        // letters_used only looks at which buckets are non-empty, not at what is inside.
        let got = letters_used(&buckets(&[
            (0, &["x"]),
            (2, &["Cat"]),
            (3, &[""]),
            (12, &["?"]),
            (13, &["M"]),
            (24, &["é"]),
            (25, &["Zed"]),
        ]));
        check_eq(got, "acdmnyz")
    });
}
