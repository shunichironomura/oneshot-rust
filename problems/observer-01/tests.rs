use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::judge::{case, check, check_eq};
use crate::user::{Subject, Subscription};

/// What the listeners were called with: (name of the listener, message).
type Log = Rc<RefCell<Vec<(&'static str, &'static str)>>>;
type Slot = Rc<RefCell<Option<Subscription>>>;

/// Every message the tests send.
const MESSAGES: [&str; 12] =
    ["hi", "bye", "1", "2", "3", "4", "", "ünï 🦀", "ping", "echo", "nested", "after"];

/// The message as a `&'static str`, so that logging does not allocate.
fn intern(msg: &str) -> &'static str {
    MESSAGES.into_iter().find(|&m| m == msg).unwrap_or("(unexpected message)")
}

fn new_log() -> Log {
    Rc::new(RefCell::new(Vec::new()))
}

/// Takes the entries out of `log`.
fn drain(log: &Log) -> Vec<(&'static str, &'static str)> {
    log.take()
}

fn empty_slot() -> Slot {
    Rc::new(RefCell::new(None))
}

/// A listener that appends `(name, msg)` to `log`.
fn logger(log: &Log, name: &'static str) -> impl Fn(&str) + 'static {
    let log = Rc::clone(log);
    move |msg| log.borrow_mut().push((name, intern(msg)))
}

/// Like `logger`, and it also holds a clone of `token` for as long as it lives.
fn tracked(log: &Log, name: &'static str, token: &Rc<()>) -> impl Fn(&str) + 'static {
    let (log, token) = (Rc::clone(log), Rc::clone(token));
    move |msg| {
        let _ = &token;
        log.borrow_mut().push((name, intern(msg)));
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
        check_eq(drain(&log), [("a", "hi"), ("b", "hi"), ("b", "bye")])
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
                l.borrow_mut().push(("once", intern(msg)));
                left.set(Some(s.listener_count()));
            })
        };
        *slot.borrow_mut() = Some(once);
        let _c = subject.subscribe(logger(&log, "c"));
        check_eq(subject.notify("1"), 3)?;
        check_eq(left.get(), Some(2))?;
        check_eq(subject.notify("2"), 2)?;
        check_eq(subject.listener_count(), 2)?;
        check_eq(drain(&log), [("a", "1"), ("once", "1"), ("c", "1"), ("a", "2"), ("c", "2")])
    });
    case("hidden-1", || {
        // No listeners, a subscription dropped at once, and the order after unsubscribing.
        let log = new_log();
        let subject = Subject::new();
        check_eq((subject.listener_count(), subject.notify("hi")), (0, 0))?;
        let _ = subject.subscribe(logger(&log, "gone"));
        check_eq((subject.listener_count(), subject.notify("hi")), (0, 0))?;
        let a = subject.subscribe(logger(&log, "a"));
        let b = subject.subscribe(logger(&log, "b"));
        let _c = subject.subscribe(logger(&log, "c"));
        let _a2 = subject.subscribe(logger(&log, "a"));
        drop(b);
        let _d = subject.subscribe(logger(&log, "d"));
        check_eq(subject.notify(""), 4)?;
        drop(a);
        let _e = subject.subscribe(logger(&log, "e"));
        check_eq(subject.listener_count(), 4)?;
        check_eq(subject.notify("ünï 🦀"), 4)?;
        check_eq(
            drain(&log),
            [
                ("a", ""),
                ("c", ""),
                ("a", ""),
                ("d", ""),
                ("c", "ünï 🦀"),
                ("a", "ünï 🦀"),
                ("d", "ünï 🦀"),
                ("e", "ünï 🦀"),
            ],
        )
    });
    case("hidden-2", || {
        // The subject does not keep a listener alive.
        let log = new_log();
        let token = Rc::new(());
        let subject = Subject::new();
        let sub = subject.subscribe(tracked(&log, "t", &token));
        check_eq(Rc::strong_count(&token), 2)?;
        drop(sub);
        check_eq(Rc::strong_count(&token), 1)?;
        let sub = subject.subscribe(tracked(&log, "t", &token));
        check_eq(subject.notify("1"), 1)?;
        drop(sub);
        check_eq(Rc::strong_count(&token), 1)?;
        check_eq((subject.listener_count(), subject.notify("2")), (0, 0))?;
        check_eq(drain(&log), [("t", "1")])?;

        // A listener that holds an Rc of its own subject.
        let shared = Rc::new(Subject::new());
        let weak = Rc::downgrade(&shared);
        let counts = Rc::new(RefCell::new(Vec::new()));
        let sub = {
            let (s, counts) = (Rc::clone(&shared), Rc::clone(&counts));
            shared.subscribe(move |_| counts.borrow_mut().push(s.listener_count()))
        };
        drop(shared);
        let alive = weak.upgrade().ok_or("the listener holds an Rc of the subject")?;
        check_eq(alive.notify("ping"), 1)?;
        drop(alive);
        drop(sub);
        check(weak.upgrade().is_none(), "the subject should be freed with its last listener")?;
        check_eq(counts.take(), [1])?;

        // A subscription that outlives its subject.
        let short = Subject::new();
        let sub = short.subscribe(tracked(&log, "t", &token));
        drop(short);
        drop(sub);
        check_eq(Rc::strong_count(&token), 1)?;
        check_eq(drain(&log), [])
    });
    case("hidden-3", || {
        // A listener that subscribes a new listener every time it is called.
        const NAMES: [&str; 2] = ["n1", "n2"];
        let log = new_log();
        let subject = Rc::new(Subject::new());
        let spawned: Rc<RefCell<Vec<Subscription>>> = Rc::new(RefCell::new(Vec::new()));
        let _a = subject.subscribe(logger(&log, "a"));
        let spawner = {
            let (s, l, spawned) = (Rc::clone(&subject), Rc::clone(&log), Rc::clone(&spawned));
            subject.subscribe(move |_| {
                let name = NAMES[spawned.borrow().len()];
                let sub = s.subscribe(logger(&l, name));
                spawned.borrow_mut().push(sub);
            })
        };
        let _b = subject.subscribe(logger(&log, "b"));
        check_eq(subject.notify("1"), 3)?;
        check_eq(subject.listener_count(), 4)?;
        check_eq(subject.notify("2"), 4)?;
        check_eq(subject.listener_count(), 5)?;
        drop(spawner);
        spawned.borrow_mut().remove(0);
        check_eq(subject.notify("3"), 3)?;
        check_eq(
            drain(&log),
            [
                ("a", "1"),
                ("b", "1"),
                ("a", "2"),
                ("b", "2"),
                ("n1", "2"),
                ("a", "3"),
                ("b", "3"),
                ("n2", "3"),
            ],
        )
    });
    case("hidden-4", || {
        // A listener drops the subscriptions of an earlier and a later listener.
        let log = new_log();
        let token = Rc::new(());
        let subject = Subject::new();
        let (a, c) = (empty_slot(), empty_slot());
        *a.borrow_mut() = Some(subject.subscribe(logger(&log, "a")));
        let _killer = {
            let (a, c, l) = (Rc::clone(&a), Rc::clone(&c), Rc::clone(&log));
            subject.subscribe(move |msg| {
                l.borrow_mut().push(("killer", intern(msg)));
                drop(a.borrow_mut().take());
                drop(c.borrow_mut().take());
            })
        };
        *c.borrow_mut() = Some(subject.subscribe(tracked(&log, "c", &token)));
        let _d = subject.subscribe(logger(&log, "d"));
        check_eq((subject.listener_count(), Rc::strong_count(&token)), (4, 2))?;
        check_eq(subject.notify("1"), 3)?;
        check_eq((subject.listener_count(), Rc::strong_count(&token)), (2, 1))?;
        check_eq(subject.notify("2"), 2)?;
        check_eq(drain(&log), [("a", "1"), ("killer", "1"), ("d", "1"), ("killer", "2"), ("d", "2")])
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
                l.borrow_mut().push(("r", intern(msg)));
                if msg == "bye" {
                    drop(me.borrow_mut().take());
                }
                if msg == "echo" || msg == "bye" {
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
        check_eq(subject.notify("after"), 2)?;
        check_eq(subject.listener_count(), 2)?;
        check_eq(counts.take(), [3, 3, 2, 2])?;
        check_eq(
            drain(&log),
            [
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
                ("a", "after"),
                ("c", "after"),
            ],
        )?;
        drop(subject);
        check(weak.upgrade().is_none(), "the subject should be freed")
    });
    case("hidden-6", || {
        // Listeners that own the subscriptions of other listeners.
        let log = new_log();
        let token = Rc::new(());
        let subject = Subject::new();
        let inner = subject.subscribe(tracked(&log, "inner", &token));
        let outer = {
            let l = Rc::clone(&log);
            subject.subscribe(move |msg| {
                let _ = &inner;
                l.borrow_mut().push(("outer", intern(msg)));
            })
        };
        check_eq((subject.listener_count(), subject.notify("1")), (2, 2))?;
        drop(outer);
        check_eq(Rc::strong_count(&token), 1)?;
        check_eq((subject.listener_count(), subject.notify("2")), (0, 0))?;

        // The same chain, dropped by a third listener during a notify.
        let y = subject.subscribe(tracked(&log, "y", &token));
        let x = {
            let l = Rc::clone(&log);
            subject.subscribe(move |msg| {
                let _ = &y;
                l.borrow_mut().push(("x", intern(msg)));
            })
        };
        let x_slot = Rc::new(RefCell::new(Some(x)));
        let _z = {
            let (x_slot, l) = (Rc::clone(&x_slot), Rc::clone(&log));
            subject.subscribe(move |msg| {
                l.borrow_mut().push(("z", intern(msg)));
                drop(x_slot.borrow_mut().take());
            })
        };
        check_eq((subject.listener_count(), subject.notify("3")), (3, 3))?;
        check_eq((subject.listener_count(), Rc::strong_count(&token)), (1, 1))?;
        check_eq(subject.notify("4"), 1)?;
        check_eq(
            drain(&log),
            [("inner", "1"), ("outer", "1"), ("y", "3"), ("x", "3"), ("z", "3"), ("z", "4")],
        )
    });
}
