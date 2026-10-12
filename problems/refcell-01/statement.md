A `LimitTracker` keeps an eye on how much of a quota has been used and sends a warning through a
`Messenger` when the usage gets close to the limit. The `Messenger` trait and `LimitTracker` are
already written in the editor. To test the tracker without sending real messages, write `Recorder`,
a mock messenger that simply remembers every message it receives. The fields of `Recorder` are up to
you.

```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> { /* given */ }

impl<'a, T: Messenger> LimitTracker<'a, T> {
    pub fn new(messenger: &'a T, max: usize) -> Self;  // given
    pub fn set_value(&mut self, value: usize);         // given
}

pub struct Recorder { /* up to you */ }

impl Recorder {
    pub fn new() -> Recorder;
    pub fn messages(&self) -> Vec<String>;
}

impl Messenger for Recorder { /* send */ }
```

- `set_value` sends at most one message per call: an error from 100% of `max`, an urgent warning
  from 90%, and a warning from 75%.
- `Recorder::new` creates a recorder with no messages. Its `send` records `msg`, whatever it is (not
  only the tracker's messages). `messages` returns a copy of all the recorded messages, oldest
  first.
- The tests call `messages` while trackers still hold a reference to the recorder, and let several
  trackers share one recorder.

Do not change the `Messenger` trait, `LimitTracker` or the given signatures.

### Examples

```rust
let recorder = Recorder::new();
recorder.send("hello");
let mut tracker = LimitTracker::new(&recorder, 100);
tracker.set_value(80); // 80%: warning
tracker.set_value(95); // 95%: urgent warning
tracker.set_value(10); // 10%: nothing
// recorder.messages() == [
//     "hello",
//     "Warning: You've used up over 75% of your quota!",
//     "Urgent warning: You've used up over 90% of your quota!",
// ]
```
