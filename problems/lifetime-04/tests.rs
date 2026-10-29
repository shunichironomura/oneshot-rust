use crate::judge::{case, check, check_eq};
use crate::user::strip_tag;

pub fn run() {
    case("example-1", || {
        let line = String::from("[INFO]  server started");
        let message;
        {
            let tag = String::from("[INFO]");
            message = strip_tag(&line, &tag);
        }
        check_eq(message, "server started")
    });
    case("case-1", || {
        // Without the tag at the very start, `line` itself comes back.
        let line = String::from("  [INFO] indented");
        let got = strip_tag(&line, "[INFO]");
        check(std::ptr::eq(got, line.as_str()), "should return `line` itself")?;
        check_eq(strip_tag("note: [INFO] later", "[INFO]"), "note: [INFO] later")?;
        check_eq(strip_tag("[info] lower case", "[INFO]"), "[info] lower case")
    });
    case("case-2", || {
        check_eq(strip_tag("[WARN]", "[WARN]"), "")?;
        check_eq(strip_tag("[WARN] \t ", "[WARN]"), "")?;
        check_eq(strip_tag("[WARN]disk full", "[WARN]"), "disk full")?;
        check_eq(strip_tag("[WARN] two  words  ", "[WARN]"), "two  words  ")?;
        check_eq(strip_tag("[A][A] twice", "[A]"), "[A] twice")
    });
    case("case-3", || {
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
}
