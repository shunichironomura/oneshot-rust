`rx.iter()` (or `for n in rx`) ends only when **every** `Sender` has been dropped. The clones moved
into the threads are dropped when the threads finish, but the original `tx` lives in `fan_in`
itself. Without `drop(tx)`, the loop waits for a message that can never come: Miri reports a
deadlock, even for an empty `groups` where no thread was spawned at all. The threads may finish in
any order, so the result must be sorted.

### Alternatives

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
