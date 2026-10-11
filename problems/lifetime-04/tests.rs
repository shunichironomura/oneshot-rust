use crate::judge::{case, check, check_eq};
use crate::user::{first_word, pick, strip_tag};

pub fn run() {
    case("sample-1", || {
        let line = String::from("[INFO]  server started");
        let message;
        {
            let tag = String::from("[INFO]");
            message = strip_tag(&line, &tag);
        }
        check_eq(message, "server started")
    });
    case("sample-2", || {
        let left = String::from("left");
        let right = String::from("right");
        check_eq(pick(&left, &right, true), "left")?;
        check_eq(pick(&left, &right, false), "right")
    });
    case("sample-3", || {
        let owned = String::from("Rust is fun");
        check_eq(first_word("  hello world"), "hello")?;
        check_eq(first_word(&owned), "Rust")
    });
    case("hidden-1", || {
        // Without the tag at the very start, `line` itself comes back.
        let line = String::from("  [INFO] indented");
        let got = strip_tag(&line, "[INFO]");
        check(std::ptr::eq(got, line.as_str()), "should return `line` itself")?;
        check_eq(strip_tag("note: [INFO] later", "[INFO]"), "note: [INFO] later")?;
        check_eq(strip_tag("[info] lower case", "[INFO]"), "[info] lower case")
    });
    case("hidden-2", || {
        check_eq(strip_tag("[WARN]", "[WARN]"), "")?;
        check_eq(strip_tag("[WARN] \t ", "[WARN]"), "")?;
        check_eq(strip_tag("[WARN]disk full", "[WARN]"), "disk full")?;
        check_eq(strip_tag("[WARN] two  words  ", "[WARN]"), "two  words  ")?;
        check_eq(strip_tag("[A][A] twice", "[A]"), "[A] twice")
    });
    case("hidden-3", || {
        let log = String::from("[INFO] booting\n[WARN]  disk almost full\n[INFO] ready");
        let mut messages = Vec::new();
        for (line, name) in log.lines().zip(["[INFO]", "[WARN]", "[INFO]"]) {
            let tag = String::from(name);
            messages.push(strip_tag(line, &tag));
        }
        check_eq(&messages, &vec!["booting", "disk almost full", "ready"])?;
        let start = log.as_ptr() as usize;
        let inside = messages.iter().all(|m| {
            let p = m.as_ptr() as usize;
            start <= p && p + m.len() <= start + log.len()
        });
        check(inside, "the messages should point into the log")
    });
    case("hidden-4", || {
        let empty = String::new();
        let word = String::from("word");
        check_eq(pick(&empty, &word, true), "")?;
        check_eq(pick(&word, &empty, false), "")?;
        check(std::ptr::eq(pick(&word, &empty, true), word.as_str()), "should return `a` itself")?;
        check(std::ptr::eq(pick(&empty, &word, false), word.as_str()), "should return `b` itself")
    });
    case("hidden-5", || {
        let info = String::from("[INFO] all good");
        let warn = String::from("[WARN]  low battery");
        let message;
        {
            let tag = String::from("[WARN]");
            message = strip_tag(pick(&info, &warn, false), &tag);
        }
        check_eq(message, "low battery")
    });
    case("hidden-6", || {
        check_eq(first_word(""), "")?;
        check_eq(first_word(" \t\n "), "")?;
        check_eq(first_word("\tone\ntwo"), "one")?;
        check_eq(first_word("  日本語 テキスト"), "日本語")?;
        let solo = String::from("solo");
        check(std::ptr::eq(first_word(&solo), solo.as_str()), "should be a slice of `s`")
    });
}
