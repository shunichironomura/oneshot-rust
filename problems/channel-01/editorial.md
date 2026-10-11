`send` takes its argument **by value**, so after `tx.send(item)` the variable `item` has been
moved, and pushing `item` in the error branch is a compile error (use of a moved value). The value
is not lost, though: when the receiver is gone, `send` returns `Err(SendError(item))`, and `.0`
hands it back. Cloning is not an option, because `T` has no bounds (the tests use a type that is
neither `Clone` nor `Copy`). Once the receiver is dropped, every later `send` fails too, and those
items must be returned as well: a loop that simply `break`s at the first error loses them.

`try_recv` never blocks and has two kinds of failure: `TryRecvError::Empty` (nothing yet, but a
sender may still send) and `TryRecvError::Disconnected` (nothing left and every sender is gone).
Messages queued before the last sender was dropped are still delivered first, so `Disconnected` is
only reported once the queue is empty. `recv()` or `rx.iter()` would block forever on a live but
empty channel.

In `fan_in`, `rx.iter()` (or `for n in rx`) ends only when **every** `Sender` has been dropped. The
clones moved into the threads are dropped when the threads finish, but the original `tx` lives in
`fan_in` itself. Without `drop(tx)`, the loop waits for a message that can never come: Miri reports a
deadlock, even for an empty `groups` where no thread was spawned at all. The threads may finish in
any order, so the result must be sorted.

### Alternatives

`send_all` as one iterator chain. `Result::err` keeps only the failures, and the `SendError(item)`
pattern unpacks the returned value.

```rust
use std::sync::mpsc::SendError;

pub fn send_all<T>(tx: &Sender<T>, items: Vec<T>) -> Vec<T> {
    items
        .into_iter()
        .filter_map(|item| tx.send(item).err())
        .map(|SendError(item)| item)
        .collect()
}
```

`drain_ready` can let the loop produce the flag: `break` with a value ends the loop and gives that
value to the `let`. `TryRecvError` implements `PartialEq`, so it can be compared with `==`.

```rust
pub fn drain_ready<T>(rx: &Receiver<T>) -> (Vec<T>, bool) {
    let mut ready = Vec::new();
    let disconnected = loop {
        match rx.try_recv() {
            Ok(value) => ready.push(value),
            Err(e) => break e == TryRecvError::Disconnected,
        }
    };
    (ready, disconnected)
}
```

With `thread::scope`, every thread is joined when the scope ends. All numbers are then already in
the channel, which the unbounded channel allows, so after dropping `tx` they can be collected without
waiting.

```rust
pub fn fan_in(groups: Vec<Vec<u32>>) -> Vec<u32> {
    let (tx, rx) = mpsc::channel();
    thread::scope(|s| {
        for group in groups {
            let tx = tx.clone();
            s.spawn(move || {
                for n in group {
                    tx.send(n).unwrap();
                }
            });
        }
    });
    drop(tx);
    let mut all: Vec<u32> = rx.iter().collect();
    all.sort();
    all
}
```

You can also count the numbers first and receive exactly that many with `rx.iter().take(total)`,
which never waits for the senders to be dropped. It works here, but it relies on knowing the total in
advance; dropping `tx` is the general way to end a receive loop.
