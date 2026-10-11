Implement three small functions around the channels of `std::sync::mpsc`.

```rust
use std::sync::mpsc::{Receiver, Sender};

pub fn send_all<T>(tx: &Sender<T>, items: Vec<T>) -> Vec<T>
pub fn drain_ready<T>(rx: &Receiver<T>) -> (Vec<T>, bool)
pub fn fan_in(groups: Vec<Vec<u32>>) -> Vec<u32>
```

- `send_all` sends `items` through `tx`, in order. It returns the items that could not be delivered
  because the receiver is gone, in their original order. If every item was sent, it returns an empty
  `Vec`.
- `drain_ready` takes every message that is already waiting in the channel, **without blocking**,
  and returns them in the order they arrived. The `bool` is `true` if, after draining, the channel is
  disconnected, that is, every `Sender` of the channel (clones included) has been dropped. Otherwise
  it is `false`.
- `fan_in` creates one channel and spawns one thread per group. Each thread sends the numbers of its
  group through its own clone of the channel's `Sender`. `fan_in` receives every number, joins all the
  threads, and returns the numbers sorted in ascending order, keeping duplicates. `groups` may be
  empty, and so may any group.

Do not change the signatures.

### Examples

```rust
let (tx, rx) = channel();
drop(rx);
send_all(&tx, vec![1, 2])          // vec![1, 2]: nobody is listening

let (tx, rx) = channel();
tx.send(1).unwrap();
tx.send(2).unwrap();
drain_ready(&rx)                   // (vec![1, 2], false)
drop(tx);
drain_ready(&rx)                   // (vec![], true)

fan_in(vec![vec![3, 1], vec![2]])  // vec![1, 2, 3]
```
