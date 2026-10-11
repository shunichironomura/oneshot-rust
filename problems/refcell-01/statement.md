A `LimitTracker` keeps an eye on how much of a quota has been used and sends a warning through a
`Messenger` when the usage gets close to the limit. To test it without sending real messages, also
write `Recorder`, a mock messenger that simply remembers every message it receives.

The `Messenger` trait and the fields of `LimitTracker` are already in the editor. The fields of
`Recorder` are up to you.

```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T: Messenger> LimitTracker<'a, T> {
    pub fn new(messenger: &'a T, max: usize) -> Self;
    pub fn set_value(&mut self, value: usize);
}

pub struct Recorder { /* up to you */ }

impl Recorder {
    pub fn new() -> Recorder;
    pub fn messages(&self) -> Vec<String>;
}

impl Messenger for Recorder { /* send */ }
```

- `LimitTracker::new` creates a tracker that reports to `messenger` and whose quota is `max`. It
  sends nothing.
- `set_value` stores `value` as the amount used so far. Then it sends the message of the first row
  of this table that applies, so **at most one** message per call:

| Condition | Message sent |
| --- | --- |
| `value` is at least `max` | `Error: You are over your quota!` |
| `value` is at least 90% of `max` | `Urgent warning: You've used up over 90% of your quota!` |
| `value` is at least 75% of `max` | `Warning: You've used up over 75% of your quota!` |
| otherwise | nothing |

- The percentages are exact. For example, with `max = 20`, a `value` of `15` is exactly 75% and
  sends the warning, while `14` sends nothing.
- Each call decides from the new `value` alone. If that gives the same message as the previous
  call, the message is sent again.
- `1 <= max <= 1_000_000` and `value <= 1_000_000`.
- `Recorder::new` creates a recorder with no messages. Its `send` records `msg`, whatever it is (not
  only the three messages above). `messages` returns a copy of all the recorded messages, oldest
  first.
- The tests call `messages` while trackers still hold a reference to the recorder, let several
  trackers share one recorder, and also use `LimitTracker` with messenger types of their own.

Do not change the `Messenger` trait or the given signatures.

### Examples

```rust
let recorder = Recorder::new();
let mut tracker = LimitTracker::new(&recorder, 100);
tracker.set_value(80); // 80%: warning
tracker.set_value(95); // 95%: urgent warning
tracker.set_value(10); // 10%: nothing
// recorder.messages() == [
//     "Warning: You've used up over 75% of your quota!",
//     "Urgent warning: You've used up over 90% of your quota!",
// ]
```

```rust
let recorder = Recorder::new();
recorder.send("hello");
let mut tracker = LimitTracker::new(&recorder, 20);
tracker.set_value(14); // 70%: nothing
tracker.set_value(15); // exactly 75%: warning
tracker.set_value(25); // over the quota: error
// recorder.messages() == [
//     "hello",
//     "Warning: You've used up over 75% of your quota!",
//     "Error: You are over your quota!",
// ]
```
