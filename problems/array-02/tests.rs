use crate::judge::{case, check, check_eq};
use crate::user::bucket_by_initial;

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
        check_eq(
            bucket_by_initial(&["bob", "Al", "42", "éclair", "", "beth"]),
            buckets(&[(0, &["Al"]), (1, &["bob", "beth"])]),
        )
    });
    case("example-2", || {
        check_eq(
            bucket_by_initial(&["zoo", "Yak", "zebra"]),
            buckets(&[(24, &["Yak"]), (25, &["zoo", "zebra"])]),
        )
    });
    case("case-1", || {
        // No words at all, and only words that are ignored (words are not trimmed).
        let got = bucket_by_initial(&[]);
        check(got.iter().all(|b| b.is_empty()), "no words: every bucket should be empty")?;
        let got = bucket_by_initial(&["", "7up", " ant", "\tbee", "_id", "-x"]);
        check(got.iter().all(|b| b.is_empty()), "every word should be ignored")
    });
    case("case-2", || {
        // Case does not matter for the bucket, but the spelling, order and duplicates are kept.
        check_eq(
            bucket_by_initial(&["Bea", "bob", "ann", "BOB", "bob", "Al"]),
            buckets(&[(0, &["ann", "Al"]), (1, &["Bea", "bob", "BOB", "bob"])]),
        )
    });
    case("case-3", || {
        // The ASCII characters right next to the letters are not letters.
        check_eq(
            bucket_by_initial(&["@home", "zulu", "[x]", "Apex", "`tick`", "{y}", "Zed", "alpha"]),
            buckets(&[(0, &["Apex", "alpha"]), (25, &["zulu", "Zed"])]),
        )
    });
    case("case-4", || {
        // Non-ASCII first characters are ignored, even when they are letters, and even when
        // their Unicode lowercase starts with an ASCII letter (U+0130 -> "i\u{307}", U+212A -> "k").
        check_eq(
            bucket_by_initial(&[
                "šálek", "ñu", "Ωmega", "ｂｉｇ", "日本", "🦀rust", "ok", "Éa", "\u{130}zmir", "\u{212A}elvin",
            ]),
            buckets(&[(14, &["ok"])]),
        )
    });
}
