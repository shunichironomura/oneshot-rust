`send` takes its argument **by value**, so after `tx.send(item)` the variable `item` has been
moved, and pushing `item` in the error branch is a compile error (use of a moved value). The value
is not lost, though: when the receiver is gone, `send` returns `Err(SendError(item))`, and `.0`
hands it back. Cloning is not an option, because `T` has no bounds. Once the receiver is dropped,
every later `send` fails too, and those items must be returned as well: a loop that simply `break`s
at the first error loses them.

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
