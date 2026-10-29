use crate::judge::{case, check, check_eq};
use crate::user::{bucket_by_initial, letters_used};

pub fn run() {
    case("example-1", || {
        let got = bucket_by_initial(&["bob", "Al", "42", "éclair", "", "beth"]);
        let mut expected: [Vec<String>; 26] = [const { Vec::new() }; 26];
        expected[0] = vec!["Al".to_string()];
        expected[1] = vec!["bob".to_string(), "beth".to_string()];
        check_eq(&got, &expected)?;
        check_eq(letters_used(&got), "ab")
    });
    case("example-2", || {
        check_eq(letters_used(&bucket_by_initial(&["zoo", "Yak", "zebra"])), "yz")
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
        let got = bucket_by_initial(&["Bea", "bob", "ann", "BOB", "bob", "Al"]);
        check_eq(letters_used(&got), "ab")?;
        check_eq(&got[0], &["ann", "Al"])?;
        check_eq(&got[1], &["Bea", "bob", "BOB", "bob"])
    });
    case("case-3", || {
        // The ASCII characters right next to the letters are not letters.
        let got = bucket_by_initial(&["@home", "zulu", "[x]", "Apex", "`tick`", "{y}", "Zed", "alpha"]);
        check_eq(letters_used(&got), "az")?;
        check_eq(&got[0], &["Apex", "alpha"])?;
        check_eq(&got[25], &["zulu", "Zed"])
    });
    case("case-4", || {
        // Non-ASCII first characters are ignored, even when they are letters, and even when
        // their Unicode lowercase starts with an ASCII letter (U+0130 -> "i\u{307}", U+212A -> "k").
        let got = bucket_by_initial(&[
            "šálek", "ñu", "Ωmega", "ｂｉｇ", "日本", "🦀rust", "ok", "Éa", "\u{130}zmir", "\u{212A}elvin",
        ]);
        check_eq(letters_used(&got), "o")?;
        check_eq(&got[14], &["ok"])
    });
    case("case-5", || {
        // letters_used only looks at which buckets are non-empty, not at what is inside.
        let mut buckets: [Vec<String>; 26] = [const { Vec::new() }; 26];
        check_eq(letters_used(&buckets), "")?;
        buckets[0].push("x".to_string());
        buckets[2].push("Cat".to_string());
        buckets[3].push(String::new());
        buckets[12].push("?".to_string());
        buckets[13].push("M".to_string());
        buckets[24].push("é".to_string());
        buckets[25].push("Zed".to_string());
        check_eq(letters_used(&buckets), "acdmnyz")
    });
}
