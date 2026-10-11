use std::cell::RefCell;
use std::rc::Rc;

use crate::judge::{case, check, check_eq};
use crate::user::{Lock, Log, keep_first, release_all, swap_in, with_lock};

fn new_log() -> Log {
    Rc::new(RefCell::new(Vec::new()))
}

/// Returns the entries logged since the last call, and clears the log.
fn drain(log: &Log) -> Vec<String> {
    std::mem::take(&mut *log.borrow_mut())
}

/// Appends `entry` to `log`, as the closures passed to `with_lock` do.
fn note(log: &Log, entry: &str) {
    log.borrow_mut().push(entry.to_string());
}

pub fn run() {
    case("sample-1", || {
        let log = new_log();
        let a = Lock::acquire("a", &log);
        let b = Lock::acquire("b", &log);
        let c = Lock::acquire("c", &log);
        check_eq(drain(&log), ["acquire a", "acquire b", "acquire c"])?;
        release_all(vec![a, b, c]);
        check_eq(drain(&log), ["release c", "release b", "release a"])
    });
    case("sample-2", || {
        let log = new_log();
        let mut x = Lock::acquire("x", &log);
        check_eq(drain(&log), ["acquire x"])?;
        swap_in(&mut x, "y");
        check_eq(drain(&log), ["acquire y", "release x"])?;
        check_eq(x.name(), "y")?;
        drop(x);
        check_eq(drain(&log), ["release y"])
    });
    case("sample-3", || {
        let log = new_log();
        let a = Lock::acquire("a", &log);
        let b = Lock::acquire("b", &log);
        let c = Lock::acquire("c", &log);
        drain(&log);
        let first = keep_first(vec![a, b, c]);
        check_eq(drain(&log), ["release b", "release c"])?;
        let first = first.ok_or("keep_first returned None")?;
        check_eq(first.name(), "a")?;
        drop(first);
        check_eq(drain(&log), ["release a"])?;
        let n = with_lock("m", &log, || {
            note(&log, "work");
            42
        });
        check_eq(n, 42)?;
        check_eq(drain(&log), ["acquire m", "work", "release m"])
    });
    case("hidden-1", || {
        // Empty and one-element vectors.
        let log = new_log();
        release_all(Vec::new());
        check(keep_first(Vec::new()).is_none(), "keep_first of an empty Vec should be None")?;
        let s = Lock::acquire("s", &log);
        let kept = keep_first(vec![s]);
        check_eq(drain(&log), ["acquire s"])?;
        check_eq(kept.as_ref().map(Lock::name), Some("s"))?;
        drop(kept);
        check_eq(drain(&log), ["release s"])
    });
    case("hidden-2", || {
        // The Vec order is not the order of acquisition, and the locks write to two logs.
        let (log1, log2) = (new_log(), new_log());
        let p = Lock::acquire("p", &log1);
        let q = Lock::acquire("q", &log2);
        let r = Lock::acquire("r", &log1);
        let t = Lock::acquire("t", &log1);
        check_eq(drain(&log1), ["acquire p", "acquire r", "acquire t"])?;
        check_eq(drain(&log2), ["acquire q"])?;
        release_all(vec![r, p, q, t]);
        check_eq(drain(&log1), ["release t", "release p", "release r"])?;
        check_eq(drain(&log2), ["release q"])
    });
    case("hidden-3", || {
        // keep_first releases the others in their Vec order.
        let log = new_log();
        let q = Lock::acquire("q", &log);
        let s = Lock::acquire("s", &log);
        let p = Lock::acquire("p", &log);
        drain(&log);
        let kept = keep_first(vec![q, s, p]);
        check_eq(drain(&log), ["release s", "release p"])?;
        let kept = kept.ok_or("keep_first returned None")?;
        check_eq(kept.name(), "q")?;
        drop(kept);
        check_eq(drain(&log), ["release q"])
    });
    case("hidden-4", || {
        // swap_in twice, with another lock on the same log.
        let log = new_log();
        let mut x = Lock::acquire("x", &log);
        let other = Lock::acquire("ログ", &log);
        swap_in(&mut x, "y");
        swap_in(&mut x, "z");
        check_eq(
            drain(&log),
            ["acquire x", "acquire ログ", "acquire y", "release x", "acquire z", "release y"],
        )?;
        check_eq(x.name(), "z")?;
        check_eq(other.name(), "ログ")?;
        drop(other);
        drop(x);
        check_eq(drain(&log), ["release ログ", "release z"])
    });
    case("hidden-5", || {
        // An empty name is a name like any other.
        let log = new_log();
        let e = Lock::acquire("", &log);
        check_eq(e.name(), "")?;
        drop(e);
        check_eq(drain(&log), ["acquire ", "release "])?;
        let mut f = Lock::acquire("f", &log);
        swap_in(&mut f, "");
        check_eq(f.name(), "")?;
        drop(f);
        check_eq(drain(&log), ["acquire f", "acquire ", "release f", "release "])?;
        with_lock("", &log, || note(&log, "anonymous"));
        check_eq(drain(&log), ["acquire ", "anonymous", "release "])
    });
    case("hidden-6", || {
        // Locals, swap_in on an element of a Vec, and keep_first together.
        let log = new_log();
        {
            let _a = Lock::acquire("a", &log);
            let mut v = vec![Lock::acquire("b", &log), Lock::acquire("c", &log)];
            swap_in(&mut v[0], "d");
            let kept = keep_first(v);
            check_eq(kept.as_ref().map(Lock::name), Some("d"))?;
        }
        check_eq(
            drain(&log),
            [
                "acquire a", "acquire b", "acquire c", "acquire d", "release b", "release c",
                "release d", "release a",
            ],
        )
    });
    case("hidden-7", || {
        // Nested calls, a result that is not Copy, and a lock that outlives the call.
        let log = new_log();
        let word = String::from("tea");
        let cup = with_lock("outer", &log, || {
            with_lock("inner", &log, || {
                note(&log, "brew");
                word + "!"
            })
        });
        check_eq(cup, "tea!")?;
        check_eq(
            drain(&log),
            ["acquire outer", "acquire inner", "brew", "release inner", "release outer"],
        )?;
        let kept = with_lock("short", &log, || Lock::acquire("long", &log));
        check_eq(drain(&log), ["acquire short", "acquire long", "release short"])?;
        check_eq(kept.name(), "long")?;
        drop(kept);
        check_eq(drain(&log), ["release long"])
    });
}
