use crate::judge::{case, check_eq};
use crate::user::{LimitTracker, Messenger, Recorder};

const WARNING: &str = "Warning: You've used up over 75% of your quota!";
const URGENT: &str = "Urgent warning: You've used up over 90% of your quota!";
const ERROR: &str = "Error: You are over your quota!";
const NOTHING: [&str; 0] = [];

pub fn run() {
    case("sample-1", || {
        let recorder = Recorder::new();
        recorder.send("hello");
        let mut tracker = LimitTracker::new(&recorder, 100);
        tracker.set_value(80);
        tracker.set_value(95);
        tracker.set_value(10);
        check_eq(recorder.messages(), ["hello", WARNING, URGENT])
    });
    case("hidden-1", || {
        // Repeated messages are all kept, in order.
        let recorder = Recorder::new();
        let mut tracker = LimitTracker::new(&recorder, 40);
        for value in [30, 30, 31, 36, 36, 0, 40, 41, 40] {
            tracker.set_value(value);
        }
        check_eq(recorder.messages(), [WARNING, WARNING, WARNING, URGENT, URGENT, ERROR, ERROR, ERROR])
    });
    case("hidden-2", || {
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
    case("hidden-3", || {
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
