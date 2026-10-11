use std::cell::RefCell;
use std::rc::Rc;

use crate::judge::{case, check, check_eq};
use crate::user::{Lock, Log, keep_first, release_all, swap_in};

fn new_log() -> Log {
    Rc::new(RefCell::new(Vec::new()))
}

/// Returns the entries logged since the last call, and clears the log.
fn drain(log: &Log) -> Vec<String> {
    std::mem::take(&mut *log.borrow_mut())
}

fn nothing() -> Vec<String> {
    Vec::new()
}

/// No lock keeps the log alive any more.
fn alone(log: &Log) -> Result<(), String> {
    check(Rc::strong_count(log) == 1, "the log is still kept alive")
}

pub fn run() {
    case("sample-1", || {
        let log = new_log();
        let a = Lock::acquire("a", &log);
        let b = Lock::acquire("b", &log);
        let c = Lock::acquire("c", &log);
        check_eq(drain(&log), ["acquire a", "acquire b", "acquire c"])?;
        release_all(vec![a, b, c]);
        check_eq(drain(&log), ["release c", "release b", "release a"])?;
        alone(&log)
    });
    case("sample-2", || {
        let log = new_log();
        let mut x = Lock::acquire("x", &log);
        check_eq(drain(&log), ["acquire x"])?;
        swap_in(&mut x, "y");
        check_eq(drain(&log), ["acquire y", "release x"])?;
        check_eq(x.name(), "y")?;
        drop(x);
        check_eq(drain(&log), ["release y"])?;
        alone(&log)
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
        check_eq(first.into_name(), "a")?;
        check_eq(drain(&log), nothing())?;
        alone(&log)
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
        check_eq(drain(&log), ["release s"])?;
        let t = Lock::acquire("t", &log);
        release_all(vec![t]);
        check_eq(drain(&log), ["acquire t", "release t"])?;
        alone(&log)
    });
    case("hidden-2", || {
        // The Vec order is not the order of acquisition, and the locks write to two logs.
        let (log1, log2) = (new_log(), new_log());
        let p = Lock::acquire("p", &log1);
        let q = Lock::acquire("q", &log2);
        let r = Lock::acquire("r", &log1);
        let s = Lock::acquire("s", &log2);
        let t = Lock::acquire("t", &log1);
        check_eq(drain(&log1), ["acquire p", "acquire r", "acquire t"])?;
        check_eq(drain(&log2), ["acquire q", "acquire s"])?;
        release_all(vec![r, p, s, q, t]);
        check_eq(drain(&log1), ["release t", "release p", "release r"])?;
        check_eq(drain(&log2), ["release q", "release s"])?;
        alone(&log1)?;
        alone(&log2)
    });
    case("hidden-3", || {
        // keep_first releases the others in their Vec order.
        let log = new_log();
        let locks: Vec<Lock> = ["q", "s", "p", "r"].iter().map(|n| Lock::acquire(n, &log)).collect();
        drain(&log);
        let kept = keep_first(locks);
        check_eq(drain(&log), ["release s", "release p", "release r"])?;
        let kept = kept.ok_or("keep_first returned None")?;
        check_eq(kept.name(), "q")?;
        drop(kept);
        check_eq(drain(&log), ["release q"])?;
        alone(&log)
    });
    case("hidden-4", || {
        // swap_in twice, then into_name on the last lock.
        let log = new_log();
        let mut x = Lock::acquire("x", &log);
        swap_in(&mut x, "y");
        swap_in(&mut x, "z");
        check_eq(
            drain(&log),
            ["acquire x", "acquire y", "release x", "acquire z", "release y"],
        )?;
        check_eq(x.name(), "z")?;
        check_eq(x.into_name(), "z")?;
        check_eq(drain(&log), nothing())?;
        alone(&log)
    });
    case("hidden-5", || {
        // into_name only disarms the lock it consumes.
        let log = new_log();
        let a = Lock::acquire("ログ", &log);
        let b = Lock::acquire("disk 0", &log);
        check_eq(a.into_name(), "ログ")?;
        check_eq(b.name(), "disk 0")?;
        drop(b);
        check_eq(drain(&log), ["acquire ログ", "acquire disk 0", "release disk 0"])?;
        alone(&log)
    });
    case("hidden-6", || {
        // An empty name is a name like any other.
        let log = new_log();
        let e = Lock::acquire("", &log);
        check_eq(e.name(), "")?;
        drop(e);
        check_eq(drain(&log), ["acquire ", "release "])?;
        let mut f = Lock::acquire("f", &log);
        swap_in(&mut f, "");
        check_eq(f.into_name(), "")?;
        check_eq(drain(&log), ["acquire f", "acquire ", "release f"])?;
        alone(&log)
    });
    case("hidden-7", || {
        // Locals, a Vec, swap_in on an element and keep_first together.
        let log = new_log();
        let a = Lock::acquire("a", &log);
        {
            let _b = Lock::acquire("b", &log);
            let mut v = vec![
                Lock::acquire("c", &log),
                Lock::acquire("d", &log),
                Lock::acquire("e", &log),
            ];
            swap_in(&mut v[1], "f");
            let kept = keep_first(v);
            let _g = Lock::acquire("g", &log);
            check_eq(kept.as_ref().map(Lock::name), Some("c"))?;
        }
        release_all(vec![a]);
        check_eq(
            drain(&log),
            [
                "acquire a", "acquire b", "acquire c", "acquire d", "acquire e", "acquire f",
                "release d", "release f", "release e", "acquire g", "release g", "release c",
                "release b", "release a",
            ],
        )?;
        alone(&log)
    });
}
