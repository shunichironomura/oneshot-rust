use std::cell::{Cell, RefCell};
use std::panic::{self, AssertUnwindSafe};
use std::rc::Rc;

use crate::judge::{case, check, check_eq};
use crate::user::{Defer, defer, defer_all};

type Log = Rc<RefCell<Vec<&'static str>>>;

fn new_log() -> Log {
    Rc::new(RefCell::new(Vec::new()))
}

/// An action that appends `s` to `log`.
fn push(log: &Log, s: &'static str) -> Box<dyn FnOnce()> {
    let log = Rc::clone(log);
    Box::new(move || log.borrow_mut().push(s))
}

fn entries(log: &Log) -> Vec<&'static str> {
    log.borrow().clone()
}

/// Runs `f` and returns the payload of the panic that escapes from it, or `None` if it returns.
fn panic_payload(f: impl FnOnce()) -> Option<&'static str> {
    let payload = panic::catch_unwind(AssertUnwindSafe(f)).err()?;
    Some(payload.downcast_ref::<&'static str>().copied().unwrap_or("(not a &str)"))
}

fn guard(log: &Log, s: &'static str) -> Defer<Box<dyn FnOnce()>> {
    defer(push(log, s))
}

pub fn run() {
    case("sample-1", || {
        let log = RefCell::new(Vec::new());
        {
            let _a = defer(|| log.borrow_mut().push("a"));
            let _b = defer(|| log.borrow_mut().push("b"));
            log.borrow_mut().push("body");
        }
        check_eq(log.borrow().clone(), vec!["body", "b", "a"])?;
        let g = defer(|| log.borrow_mut().push("never"));
        g.cancel();
        let g = defer(|| log.borrow_mut().push("now"));
        g.run_now();
        log.borrow_mut().push("end");
        check_eq(log.into_inner(), vec!["body", "b", "a", "now", "end"])
    });
    case("sample-2", || {
        let log = RefCell::new(Vec::new());
        let payload = panic_payload(|| {
            let _g = defer(|| log.borrow_mut().push("cleanup"));
            panic!("oops");
        });
        check_eq(payload, Some("oops"))?;
        check_eq(log.into_inner(), vec!["cleanup"])
    });
    case("sample-3", || {
        let log = new_log();
        let all = defer_all(vec![push(&log, "1"), push(&log, "2"), push(&log, "3")]);
        check(entries(&log).is_empty(), "defer_all must not run the actions yet")?;
        drop(all);
        check_eq(entries(&log), vec!["3", "2", "1"])
    });
    case("hidden-1", || {
        // The action runs once, when the last owner of the guard drops it.
        let count = Cell::new(0);
        let g = defer(|| count.set(count.get() + 1));
        let moved = g;
        check_eq(count.get(), 0)?;
        drop(moved);
        check_eq(count.get(), 1)?;
        let g = defer(|| count.set(count.get() + 10));
        g.run_now();
        check_eq(count.get(), 11)?;
        // A closure that can be called only once, and a boxed action.
        let words = RefCell::new(Vec::new());
        let words_ref = &words;
        let name = String::from("ferris");
        {
            let _g = defer(move || words_ref.borrow_mut().push(name));
            check(words.borrow().is_empty(), "the action must not run before the drop")?;
        }
        check_eq(words.into_inner(), vec![String::from("ferris")])?;
        let log = new_log();
        let guards = vec![guard(&log, "x"), guard(&log, "y")];
        guard(&log, "z").run_now();
        drop(guards);
        check_eq(entries(&log), vec!["z", "x", "y"])
    });
    case("hidden-2", || {
        // `cancel` drops the action at once, without running it.
        let token = Rc::new(());
        let t = Rc::clone(&token);
        let g = defer(move || drop(t));
        check_eq(Rc::strong_count(&token), 2)?;
        g.cancel();
        check_eq(Rc::strong_count(&token), 1)?;
        let log = new_log();
        guard(&log, "cancelled").cancel();
        check(entries(&log).is_empty(), "a cancelled action must not run")?;
        check_eq(Rc::strong_count(&log), 1)?;
        let g = guard(&log, "run");
        check_eq(Rc::strong_count(&log), 2)?;
        g.run_now();
        check_eq(Rc::strong_count(&log), 1)?;
        check_eq(entries(&log), vec!["run"])
    });
    case("hidden-3", || {
        // An empty list, cancel and run_now on the guard of defer_all.
        defer_all(Vec::new()).run_now();
        drop(defer_all(Vec::new()));
        let log = new_log();
        defer_all(vec![push(&log, "1"), push(&log, "2"), push(&log, "3")]).cancel();
        check(entries(&log).is_empty(), "cancel must run none of the actions")?;
        check_eq(Rc::strong_count(&log), 1)?;
        let all = defer_all(vec![push(&log, "a"), push(&log, "b")]);
        all.run_now();
        check_eq(entries(&log), vec!["b", "a"])?;
        check_eq(Rc::strong_count(&log), 1)
    });
    case("hidden-4", || {
        // A panic in the middle: the remaining actions still run, last to first.
        let log = new_log();
        let actions: Vec<Box<dyn FnOnce()>> = vec![
            push(&log, "1"),
            push(&log, "2"),
            push(&log, "3"),
            Box::new(|| panic!("4")),
            push(&log, "5"),
        ];
        let payload = panic_payload(move || drop(defer_all(actions)));
        check_eq(payload, Some("4"))?;
        check_eq(entries(&log), vec!["5", "3", "2", "1"])?;
        check_eq(Rc::strong_count(&log), 1)
    });
    case("hidden-5", || {
        // The first action to run panics, through run_now: the panic leaves run_now.
        let log = new_log();
        let payload = panic_payload(|| {
            let all = defer_all(vec![push(&log, "1"), push(&log, "2"), Box::new(|| panic!("3"))]);
            log.borrow_mut().push("body");
            all.run_now();
            log.borrow_mut().push("after run_now");
        });
        check_eq(payload, Some("3"))?;
        check_eq(entries(&log), vec!["body", "2", "1"])?;
        check_eq(Rc::strong_count(&log), 1)
    });
    case("hidden-6", || {
        // An action that panics on a normal drop runs once, and the guards declared before it,
        // including one from defer_all, still run while the panic unwinds.
        let count = Cell::new(0);
        let log = new_log();
        let payload = panic_payload(|| {
            let _all = defer_all(vec![push(&log, "1"), push(&log, "2")]);
            let _one = guard(&log, "one");
            let _bad = defer(|| {
                count.set(count.get() + 1);
                panic!("bad");
            });
        });
        check_eq(payload, Some("bad"))?;
        check_eq(count.get(), 1)?;
        check_eq(entries(&log), vec!["one", "2", "1"])
    });
}
