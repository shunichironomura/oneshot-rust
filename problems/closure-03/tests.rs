use crate::judge::{case, check_eq};
use crate::user::tally;

fn check_tally(events: &[&str], limit: usize, errors: usize, log: &[&str]) -> Result<(), String> {
    let (got_errors, got_log) = tally(events, limit);
    let got_log: Vec<&str> = got_log.iter().map(String::as_str).collect();
    check_eq((got_errors, got_log.as_slice()), (errors, log))
}

pub fn run() {
    case("example-1", || {
        check_tally(&["ok", "ERR a", "ERR b", "ok"], 2, 2, &["ok", "ERR a", "ERR b", "abort"])?;
        check_tally(&["ok"], 1, 0, &["ok"])
    });
    case("case-1", || {
        // With limit 1, the first error aborts at once. Events after the abort are neither
        // logged nor counted.
        check_tally(&["ERR"], 1, 1, &["ERR", "abort"])?;
        check_tally(&["ERR x", "ok", "ERR y"], 1, 1, &["ERR x", "abort"])?;
        check_tally(&["ERR a", "ERR b", "ERR c", "ok", "ERR d"], 2, 2, &["ERR a", "ERR b", "abort"])
    });
    case("case-2", || {
        // The limit is reached by the last event: "abort" is still appended.
        check_tally(&["ok", "ERR 1", "ok", "ERR 2"], 2, 2, &["ok", "ERR 1", "ok", "ERR 2", "abort"])
    });
    case("case-3", || {
        // The limit is never reached, and an empty list.
        check_tally(&["ERR a", "ok", "ERR b"], 3, 2, &["ERR a", "ok", "ERR b"])?;
        check_tally(&[], 1, 0, &[])
    });
    case("case-4", || {
        // Only a case-sensitive "ERR" at the very start counts.
        check_tally(
            &["err x", "Err", " ERR", "xERR", "ERROR", "ok", "ERR"],
            2,
            2,
            &["err x", "Err", " ERR", "xERR", "ERROR", "ok", "ERR", "abort"],
        )
    });
}
