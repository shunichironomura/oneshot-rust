use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};

use crate::judge::{case, check, check_eq};
use crate::user::{Subject, Subscription};

/// Who was called with what, in order.
type Log = Rc<RefCell<Vec<(&'static str, String)>>>;
/// Who was called, in order (for the tests where the message does not matter).
type Names = Rc<RefCell<Vec<&'static str>>>;
type Slot = Rc<RefCell<Option<Subscription>>>;

fn new_log() -> Log {
    Rc::new(RefCell::new(Vec::new()))
}

fn new_names() -> Names {
    Rc::new(RefCell::new(Vec::new()))
}

fn empty_slot() -> Slot {
    Rc::new(RefCell::new(None))
}

/// Drops the subscription in the slot, if any. A listener that unsubscribes itself holds a
/// `Weak` to its own slot, so that a failing test does not leave a cycle behind.
fn clear(slot: &Weak<RefCell<Option<Subscription>>>) {
    if let Some(slot) = slot.upgrade() {
        drop(slot.borrow_mut().take());
    }
}

/// Checks the entries of `log` and empties it.
fn check_log(log: &Log, expected: &[(&str, &str)]) -> Result<(), String> {
    let got = log.take();
    let same = got.len() == expected.len()
        && got.iter().zip(expected).all(|((name, msg), (n, m))| name == n && msg == m);
    if same { Ok(()) } else { Err(format!("expected {expected:?}, got {got:?}")) }
}

/// A listener that appends `(name, msg)` to `log`.
fn logger(log: &Log, name: &'static str) -> impl Fn(&str) + 'static {
    let log = Rc::clone(log);
    move |msg| log.borrow_mut().push((name, msg.to_string()))
}

/// A listener that appends `name` to `names`.
fn marker(names: &Names, name: &'static str) -> impl Fn(&str) + 'static {
    let names = Rc::clone(names);
    move |_| names.borrow_mut().push(name)
}

/// Like `marker`, and it also holds a clone of `token` for as long as it lives.
fn tracked(names: &Names, name: &'static str, token: &Rc<()>) -> impl Fn(&str) + 'static {
    let (names, token) = (Rc::clone(names), Rc::clone(token));
    move |_| {
        let _ = &token;
        names.borrow_mut().push(name);
    }
}

pub fn run() {
    case("sample-1", || {
        let log = new_log();
        let subject = Subject::new();
        let a = subject.subscribe(logger(&log, "a"));
        let _b = subject.subscribe(logger(&log, "b"));
        check_eq(subject.listener_count(), 2)?;
        check_eq(subject.notify("hi"), 2)?;
        drop(a);
        check_eq(subject.listener_count(), 1)?;
        check_eq(subject.notify("bye"), 1)?;
        check_log(&log, &[("a", "hi"), ("b", "hi"), ("b", "bye")])
    });
    case("sample-2", || {
        let log = new_log();
        let subject = Rc::new(Subject::new());
        let _a = subject.subscribe(logger(&log, "a"));
        // A listener that unsubscribes itself the first time it is called, then records how
        // many listeners are left.
        let slot = empty_slot();
        let left = Rc::new(Cell::new(None));
        let once = {
            let (me, s, l, left) =
                (Rc::downgrade(&slot), Rc::clone(&subject), Rc::clone(&log), Rc::clone(&left));
            subject.subscribe(move |msg| {
                clear(&me);
                l.borrow_mut().push(("once", msg.to_string()));
                left.set(Some(s.listener_count()));
            })
        };
        *slot.borrow_mut() = Some(once);
        let _c = subject.subscribe(logger(&log, "c"));
        check_eq(subject.notify("1"), 3)?;
        check_eq(left.get(), Some(2))?;
        check_eq(subject.notify("2"), 2)?;
        check_eq(subject.listener_count(), 2)?;
        check_log(&log, &[("a", "1"), ("once", "1"), ("c", "1"), ("a", "2"), ("c", "2")])
    });
    case("hidden-1", || {
        // A subscription dropped at once, the order after unsubscribing, and unusual messages.
        let names = new_names();
        let subject = Subject::new();
        let _ = subject.subscribe(marker(&names, "gone"));
        check_eq((subject.listener_count(), subject.notify("hi")), (0, 0))?;
        let a = subject.subscribe(marker(&names, "a"));
        let b = subject.subscribe(marker(&names, "b"));
        let messages = Rc::new(RefCell::new(Vec::new()));
        let _c = {
            let (n, m) = (Rc::clone(&names), Rc::clone(&messages));
            subject.subscribe(move |msg| {
                n.borrow_mut().push("c");
                m.borrow_mut().push(msg.to_string());
            })
        };
        drop(b);
        let _d = subject.subscribe(marker(&names, "d"));
        check_eq(subject.notify(""), 3)?;
        drop(a);
        check_eq(subject.listener_count(), 2)?;
        check_eq(subject.notify("ünï 🦀"), 2)?;
        check_eq(names.take(), ["a", "c", "d", "c", "d"])?;
        check_eq(messages.take(), ["", "ünï 🦀"])
    });
    case("hidden-2", || {
        // Dropping a subscription frees its listener at once, together with the subscriptions
        // that the listener owns.
        let names = new_names();
        let token = Rc::new(());
        let subject = Subject::new();
        let inner = subject.subscribe(tracked(&names, "inner", &token));
        let outer = {
            let n = Rc::clone(&names);
            subject.subscribe(move |_| {
                let _ = &inner;
                n.borrow_mut().push("outer");
            })
        };
        let _last = subject.subscribe(marker(&names, "last"));
        check_eq((subject.listener_count(), subject.notify("1")), (3, 3))?;
        check_eq(Rc::strong_count(&token), 2)?;
        drop(outer);
        check_eq((Rc::strong_count(&token), subject.listener_count()), (1, 1))?;
        check_eq(names.take(), ["inner", "outer", "last"])?;

        // A listener that holds an Rc of its own subject.
        let shared = Rc::new(Subject::new());
        let weak = Rc::downgrade(&shared);
        let counts = Rc::new(Cell::new(0));
        let sub = {
            let (s, counts) = (Rc::clone(&shared), Rc::clone(&counts));
            shared.subscribe(move |_| counts.set(counts.get() + s.listener_count()))
        };
        drop(shared);
        let alive = weak.upgrade().ok_or("the listener holds an Rc of the subject")?;
        check_eq(alive.notify("ping"), 1)?;
        drop(alive);
        drop(sub);
        check(weak.upgrade().is_none(), "the subject should be freed with its last listener")?;
        check_eq(counts.get(), 1)?;

        // A subscription that outlives its subject.
        let short = Subject::new();
        let sub = short.subscribe(tracked(&names, "u", &token));
        drop(short);
        drop(sub);
        check_eq(Rc::strong_count(&token), 1)
    });
    case("hidden-3", || {
        // A listener that subscribes a new listener every time it is called.
        const NAMES: [&str; 2] = ["n1", "n2"];
        let names = new_names();
        let subject = Rc::new(Subject::new());
        let spawned: Rc<RefCell<Vec<Subscription>>> = Rc::new(RefCell::new(Vec::new()));
        let _a = subject.subscribe(marker(&names, "a"));
        let spawner = {
            let (s, n, spawned) = (Rc::clone(&subject), Rc::clone(&names), Rc::clone(&spawned));
            subject.subscribe(move |_| {
                let name = NAMES[spawned.borrow().len()];
                let sub = s.subscribe(marker(&n, name));
                spawned.borrow_mut().push(sub);
            })
        };
        let _b = subject.subscribe(marker(&names, "b"));
        check_eq(subject.notify("1"), 3)?;
        check_eq(subject.listener_count(), 4)?;
        check_eq(subject.notify("2"), 4)?;
        check_eq(subject.listener_count(), 5)?;
        drop(spawner);
        drop(spawned.borrow_mut().remove(0));
        check_eq(subject.listener_count(), 3)?;
        check_eq(names.take(), ["a", "b", "a", "b", "n1"])
    });
    case("hidden-4", || {
        // A listener drops the subscriptions of an earlier and a later listener.
        let names = new_names();
        let token = Rc::new(());
        let subject = Subject::new();
        let (a, c) = (empty_slot(), empty_slot());
        *a.borrow_mut() = Some(subject.subscribe(marker(&names, "a")));
        let _killer = {
            let (a, c, n) = (Rc::clone(&a), Rc::clone(&c), Rc::clone(&names));
            subject.subscribe(move |_| {
                n.borrow_mut().push("killer");
                drop(a.borrow_mut().take());
                drop(c.borrow_mut().take());
            })
        };
        *c.borrow_mut() = Some(subject.subscribe(tracked(&names, "c", &token)));
        let _d = subject.subscribe(marker(&names, "d"));
        check_eq((subject.listener_count(), Rc::strong_count(&token)), (4, 2))?;
        check_eq(subject.notify("1"), 3)?;
        check_eq((subject.listener_count(), Rc::strong_count(&token)), (2, 1))?;
        check_eq(names.take(), ["a", "killer", "d"])
    });
    case("hidden-5", || {
        // Nested notify calls, before and after a listener unsubscribes itself.
        let names = new_names();
        let subject = Rc::new(Subject::new());
        let weak = Rc::downgrade(&subject);
        let counts = Rc::new(RefCell::new(Vec::new()));
        let _a = subject.subscribe(marker(&names, "a"));
        let slot = empty_slot();
        let r = {
            let (me, s, n, counts) =
                (Rc::downgrade(&slot), Rc::clone(&subject), Rc::clone(&names), Rc::clone(&counts));
            subject.subscribe(move |msg| {
                n.borrow_mut().push("r");
                if msg == "bye" {
                    clear(&me);
                }
                if msg != "nested" {
                    let count = s.listener_count();
                    let called = s.notify("nested");
                    counts.borrow_mut().extend([count, called]);
                }
            })
        };
        *slot.borrow_mut() = Some(r);
        let _c = subject.subscribe(marker(&names, "c"));
        check_eq(subject.notify("echo"), 3)?;
        check_eq(names.take(), ["a", "r", "a", "r", "c", "c"])?;
        check_eq(subject.notify("bye"), 3)?;
        check_eq(names.take(), ["a", "r", "a", "c", "c"])?;
        check_eq(subject.listener_count(), 2)?;
        check_eq(counts.take(), [3, 3, 2, 2])?;
        drop(subject);
        check(weak.upgrade().is_none(), "the subject should be freed")
    });
}
