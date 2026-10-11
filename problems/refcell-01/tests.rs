use std::cell::Cell;

use crate::judge::{case, check_eq};
use crate::user::{LimitTracker, Messenger, Recorder};

const WARNING: &str = "Warning: You've used up over 75% of your quota!";
const URGENT: &str = "Urgent warning: You've used up over 90% of your quota!";
const ERROR: &str = "Error: You are over your quota!";
const NOTHING: [&str; 0] = [];

/// A messenger of the harness's own: it counts the calls and keeps the last message.
struct Probe {
    calls: Cell<usize>,
    last: Cell<&'static str>,
}

impl Probe {
    fn new() -> Probe {
        Probe { calls: Cell::new(0), last: Cell::new("") }
    }
}

impl Messenger for Probe {
    fn send(&self, msg: &str) {
        self.calls.set(self.calls.get() + 1);
        let known = [WARNING, URGENT, ERROR].into_iter().find(|&m| m == msg);
        self.last.set(known.unwrap_or("(unexpected message)"));
    }
}

/// What a new tracker with quota `max` sends for `set_value(value)`: "" for nothing.
fn sent_for(max: usize, value: usize) -> Result<&'static str, String> {
    let probe = Probe::new();
    let mut tracker = LimitTracker::new(&probe, max);
    tracker.set_value(value);
    match probe.calls.get() {
        0 | 1 => Ok(probe.last.get()),
        n => Err(format!("max {max}, value {value}: sent {n} messages")),
    }
}

pub fn run() {
    case("sample-1", || {
        let recorder = Recorder::new();
        let mut tracker = LimitTracker::new(&recorder, 100);
        tracker.set_value(80);
        tracker.set_value(95);
        tracker.set_value(10);
        check_eq(recorder.messages(), [WARNING, URGENT])
    });
    case("sample-2", || {
        let recorder = Recorder::new();
        recorder.send("hello");
        let mut tracker = LimitTracker::new(&recorder, 20);
        tracker.set_value(14);
        tracker.set_value(15);
        tracker.set_value(25);
        check_eq(recorder.messages(), ["hello", WARNING, ERROR])
    });
    case("hidden-1", || {
        // Every boundary belongs to the higher level.
        check_eq(sent_for(100, 74)?, "")?;
        check_eq(sent_for(100, 75)?, WARNING)?;
        check_eq(sent_for(100, 89)?, WARNING)?;
        check_eq(sent_for(100, 90)?, URGENT)?;
        check_eq(sent_for(100, 99)?, URGENT)?;
        check_eq(sent_for(100, 100)?, ERROR)?;
        check_eq(sent_for(100, 150)?, ERROR)?;
        check_eq(sent_for(100, 1_000_000)?, ERROR)
    });
    case("hidden-2", || {
        // Quotas other than 100: rounding the percentage or the threshold gives wrong answers.
        check_eq(sent_for(1000, 749)?, "")?;
        check_eq(sent_for(1000, 750)?, WARNING)?;
        check_eq(sent_for(1000, 899)?, WARNING)?;
        check_eq(sent_for(1000, 900)?, URGENT)?;
        check_eq(sent_for(67, 50)?, "")?;
        check_eq(sent_for(7, 5)?, "")?;
        check_eq(sent_for(7, 6)?, WARNING)?;
        check_eq(sent_for(13, 9)?, "")?;
        check_eq(sent_for(13, 11)?, WARNING)?;
        check_eq(sent_for(13, 12)?, URGENT)?;
        check_eq(sent_for(29, 26)?, WARNING)?;
        check_eq(sent_for(4, 3)?, WARNING)?;
        check_eq(sent_for(1, 0)?, "")?;
        check_eq(sent_for(1, 1)?, ERROR)?;
        check_eq(sent_for(1_000_000, 749_999)?, "")?;
        check_eq(sent_for(1_000_000, 900_000)?, URGENT)?;
        check_eq(sent_for(1_000_000, 999_999)?, URGENT)
    });
    case("hidden-3", || {
        // The tracker works with any messenger, and sends at most one message per call.
        let probe = Probe::new();
        let mut tracker = LimitTracker::new(&probe, 100);
        check_eq(probe.calls.get(), 0)?;
        for value in [0, 80, 95, 120, 10, 100, 74, 90] {
            tracker.set_value(value);
        }
        check_eq((probe.calls.get(), probe.last.get()), (5, URGENT))
    });
    case("hidden-4", || {
        // The same message is sent again on every call that calls for it.
        let recorder = Recorder::new();
        let mut tracker = LimitTracker::new(&recorder, 40);
        for value in [30, 30, 31, 36, 36, 0, 40, 41, 40] {
            tracker.set_value(value);
        }
        check_eq(recorder.messages(), [WARNING, WARNING, WARNING, URGENT, URGENT, ERROR, ERROR, ERROR])
    });
    case("hidden-5", || {
        // Two trackers share one recorder, and messages() is read while both still borrow it.
        let recorder = Recorder::new();
        let other = Recorder::new();
        let mut disk = LimitTracker::new(&recorder, 10);
        let mut mail = LimitTracker::new(&recorder, 200);
        let mut spare = LimitTracker::new(&other, 10);
        disk.set_value(8);
        let before = recorder.messages();
        mail.set_value(190);
        disk.set_value(10);
        mail.set_value(100);
        check_eq(before, [WARNING])?;
        check_eq(recorder.messages(), [WARNING, URGENT, ERROR])?;
        check_eq(other.messages(), NOTHING)?;
        mail.set_value(150);
        spare.set_value(9);
        check_eq(recorder.messages(), [WARNING, URGENT, ERROR, WARNING])?;
        check_eq(other.messages(), [URGENT])
    });
    case("hidden-6", || {
        // A recorder keeps any message, in order, including empty and repeated ones.
        let recorder = Recorder::new();
        check_eq(recorder.messages(), NOTHING)?;
        recorder.send("ping");
        recorder.send("");
        recorder.send("ping");
        let messenger: &dyn Messenger = &recorder;
        messenger.send("残り 5%");
        check_eq(recorder.messages(), ["ping", "", "ping", "残り 5%"])
    });
}
