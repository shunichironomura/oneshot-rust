Sending through a channel of `std::sync::mpsc` fails when the receiver is gone. Implement
`send_all`, which sends a batch of items and gives back the ones that could not be delivered.

```rust
use std::sync::mpsc::Sender;

pub fn send_all<T>(tx: &Sender<T>, items: Vec<T>) -> Vec<T>
```

- `send_all` sends `items` through `tx`, in order.
- It returns the items that could not be delivered because the receiver is gone, in their original
  order. If every item was sent, it returns an empty `Vec`.
- `T` can be any type, including types that implement neither `Clone` nor `Copy`.

Do not change the signature.

### Examples

```rust
let (tx, rx) = channel();
drop(rx);
send_all(&tx, vec![1, 2])          // vec![1, 2]: nobody is listening
```
