use std::thread;

use crate::judge::{case, check_eq};
use crate::user::{intern, intern_calls, interned_count, resolve};

// All cases share one global table and run in this order: `sample-1` starts with an empty table,
// and the later cases intern fresh strings and compare the counts with their values before.
pub fn run() {
    case("sample-1", || {
        check_eq(intern("apple"), 0)?;
        check_eq(intern("pear"), 1)?;
        check_eq(intern("apple"), 0)?;
        check_eq(resolve(1).as_deref(), Some("pear"))?;
        check_eq(resolve(2), None)?;
        check_eq((interned_count(), intern_calls()), (2, 3))
    });
    case("hidden-1", || {
        // Exact comparison (case matters, nothing is trimmed); repeated strings still count as
        // calls, and resolve and interned_count do not.
        let n = interned_count() as u32;
        let calls = intern_calls();
        check_eq(intern("h1-red"), n)?;
        check_eq(intern("h1-green"), n + 1)?;
        check_eq(intern("H1-red"), n + 2)?;
        check_eq(intern("h1-red "), n + 3)?;
        check_eq(intern("h1-red"), n)?;
        check_eq(resolve(n + 2).as_deref(), Some("H1-red"))?;
        check_eq(resolve(n + 3).as_deref(), Some("h1-red "))?;
        check_eq(interned_count() as u32, n + 4)?;
        check_eq(intern_calls(), calls + 5)
    });
    case("hidden-2", || {
        // The empty string and non-ASCII strings; strings from earlier cases keep their ids.
        let n = interned_count() as u32;
        check_eq(intern(""), n)?;
        check_eq(intern("ねこ"), n + 1)?;
        check_eq(intern(""), n)?;
        check_eq(resolve(n).as_deref(), Some(""))?;
        check_eq(resolve(n + 1).as_deref(), Some("ねこ"))?;
        check_eq(intern("pear"), 1)?;
        check_eq(resolve(0).as_deref(), Some("apple"))
    });
    case("hidden-3", || {
        // Three threads at the same time; each word is interned by two of them.
        const WORDS: [&str; 3] = ["t-ant", "t-bee", "t-cat"];
        let n = interned_count() as u32;
        let calls = intern_calls();
        let got: Vec<[u32; 2]> = thread::scope(|s| {
            let handles: Vec<_> = (0..3)
                .map(|t| s.spawn(move || [intern(WORDS[t]), intern(WORDS[(t + 1) % 3])]))
                .collect();
            handles.into_iter().map(|h| h.join().unwrap()).collect()
        });
        let ids = [got[0][0], got[1][0], got[2][0]];
        check_eq([got[2][1], got[0][1], got[1][1]], ids)?;
        for (id, word) in ids.into_iter().zip(WORDS) {
            check_eq(resolve(id).as_deref(), Some(word))?;
        }
        let mut sorted = ids;
        sorted.sort();
        check_eq(sorted, [n, n + 1, n + 2])?;
        check_eq(interned_count() as u32, n + 3)?;
        check_eq(intern_calls(), calls + 6)
    });
    case("hidden-4", || {
        // Ids that have not been handed out. (Last, so that a panic here cannot affect the others.)
        let n = interned_count() as u32;
        check_eq(resolve(n), None)?;
        check_eq(resolve(n + 1), None)?;
        check_eq(resolve(u32::MAX), None)?;
        check_eq(intern("h4-last"), n)?;
        check_eq(resolve(n).as_deref(), Some("h4-last"))?;
        check_eq(resolve(n + 1), None)
    });
}
