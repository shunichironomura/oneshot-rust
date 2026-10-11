use std::cell::{Cell, RefCell};
use std::rc::Rc;

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
        check_eq(log.take(), [("a", "hi"), ("b", "hi"), ("b", "bye")].map(|(n, m)| (n, m.into())))
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
                (Rc::clone(&slot), Rc::clone(&subject), Rc::clone(&log), Rc::clone(&left));
            subject.subscribe(move |msg| {
                drop(me.borrow_mut().take());
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
        let expected = [("a", "1"), ("once", "1"), ("c", "1"), ("a", "2"), ("c", "2")];
        check_eq(log.take(), expected.map(|(n, m)| (n, m.into())))
    });
    case("hidden-1", || {
        // A subscription dropped at once, the order after unsubscribing, and unusual messages.
        let log = new_log();
        let subject = Subject::new();
        let _ = subject.subscribe(logger(&log, "gone"));
        check_eq((subject.listener_count(), subject.notify("hi")), (0, 0))?;
        let a = subject.subscribe(logger(&log, "a"));
        let b = subject.subscribe(logger(&log, "b"));
        let _c = subject.subscribe(logger(&log, "c"));
        drop(b);
        let _d = subject.subscribe(logger(&log, "d"));
        check_eq(subject.notify(""), 3)?;
        drop(a);
        let _e = subject.subscribe(logger(&log, "e"));
        check_eq(subject.listener_count(), 3)?;
        check_eq(subject.notify("ünï 🦀"), 3)?;
        let expected =
            [("a", ""), ("c", ""), ("d", ""), ("c", "ünï 🦀"), ("d", "ünï 🦀"), ("e", "ünï 🦀")];
        check_eq(log.take(), expected.map(|(n, m)| (n, m.into())))
    });
    case("hidden-2", || {
        // The subject keeps neither a listener nor itself alive.
        let names = new_names();
        let token = Rc::new(());
        let subject = Subject::new();
        let sub = subject.subscribe(tracked(&names, "t", &token));
        check_eq(subject.notify("1"), 1)?;
        check_eq(Rc::strong_count(&token), 2)?;
        drop(sub);
        check_eq(Rc::strong_count(&token), 1)?;
        check_eq((subject.listener_count(), subject.notify("2")), (0, 0))?;

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
        check_eq(Rc::strong_count(&token), 2)?;
        drop(sub);
        check_eq(Rc::strong_count(&token), 1)?;
        check_eq(names.take(), ["t"])
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
        spawned.borrow_mut().remove(0);
        check_eq(subject.notify("3"), 3)?;
        check_eq(names.take(), ["a", "b", "a", "b", "n1", "a", "b", "n2"])
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
        check_eq(subject.notify("2"), 2)?;
        check_eq(names.take(), ["a", "killer", "d", "killer", "d"])
    });
    case("hidden-5", || {
        // Nested notify calls, before and after a listener unsubscribes itself.
        let log = new_log();
        let subject = Rc::new(Subject::new());
        let weak = Rc::downgrade(&subject);
        let counts = Rc::new(RefCell::new(Vec::new()));
        let _a = subject.subscribe(logger(&log, "a"));
        let slot = empty_slot();
        let r = {
            let (me, s, l, counts) =
                (Rc::clone(&slot), Rc::clone(&subject), Rc::clone(&log), Rc::clone(&counts));
            subject.subscribe(move |msg| {
                l.borrow_mut().push(("r", msg.to_string()));
                if msg == "bye" {
                    drop(me.borrow_mut().take());
                }
                if msg != "nested" {
                    let count = s.listener_count();
                    let called = s.notify("nested");
                    counts.borrow_mut().extend([count, called]);
                }
            })
        };
        *slot.borrow_mut() = Some(r);
        let _c = subject.subscribe(logger(&log, "c"));
        check_eq(subject.notify("echo"), 3)?;
        check_eq(subject.notify("bye"), 3)?;
        check_eq(subject.listener_count(), 2)?;
        check_eq(counts.take(), [3, 3, 2, 2])?;
        let expected = [
            ("a", "echo"),
            ("r", "echo"),
            ("a", "nested"),
            ("r", "nested"),
            ("c", "nested"),
            ("c", "echo"),
            ("a", "bye"),
            ("r", "bye"),
            ("a", "nested"),
            ("c", "nested"),
            ("c", "bye"),
        ];
        check_eq(log.take(), expected.map(|(n, m)| (n, m.into())))?;
        drop(subject);
        check(weak.upgrade().is_none(), "the subject should be freed")
    });
    case("hidden-6", || {
        // A listener that owns the subscription of another listener.
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
        drop(outer);
        check_eq(Rc::strong_count(&token), 1)?;
        check_eq((subject.listener_count(), subject.notify("2")), (1, 1))?;
        check_eq(names.take(), ["inner", "outer", "last", "last"])
    });
}
