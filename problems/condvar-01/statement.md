A bounded blocking queue lets threads hand items to each other: producers `push` items, consumers
`pop` them in the same order, and each side waits while there is nothing it can do. Implement
`BoundedQueue<T>`. Choose its fields yourself; you may use anything from `std`. (A
`std::sync::Condvar` lets a thread sleep until another thread wakes it up.)

```rust
pub struct BoundedQueue<T> {
    // fields of your choice
}

impl<T> BoundedQueue<T> {
    pub fn new(capacity: usize) -> Self;
    pub fn push(&self, item: T) -> Result<(), T>;
    pub fn pop(&self) -> Option<T>;
    pub fn close(&self);
    pub fn len(&self) -> usize;
}
```

- `new(capacity)` creates an empty, open queue that holds at most `capacity` items. `capacity` is at
  least 1.
- `push(item)` adds `item` at the back of the queue and returns `Ok(())`. While the queue is full,
  it waits until there is room.
- If the queue is closed, `push` returns `Err(item)` at once and does not add the item, even if
  there is room. A `push` that is waiting when the queue gets closed stops waiting and returns
  `Err(item)` too. An item is never added to a closed queue.
- `pop()` removes the item at the front of the queue and returns `Some(item)`: items come out in
  the order they were added. While the queue is empty and open, it waits until there is an item or
  the queue is closed.
- The items that are in the queue when it is closed can still be popped. `pop` returns `None` only
  when the queue is closed and empty, and then it returns at once.
- `close()` closes the queue for good. Closing a closed queue again does nothing.
- `len()` returns the number of items in the queue right now. It is never more than `capacity`.
- A waiting call must not sleep forever when it can go on: a waiting `push` must notice when there
  is room, a waiting `pop` when there is an item, and every waiting call when the queue is closed.
- The harness shares one `&BoundedQueue<T>` between several threads (with `std::thread::scope`), so
  `BoundedQueue<T>` must work from several threads at once whenever `T` is `Send`. `T` does not
  have to be `Clone`, `Copy` or `Debug`.
- The harness never waits for something that cannot happen: for example, it does not pop from an
  empty queue that no thread will push to or close. (When every thread waits forever, Miri reports
  a deadlock.)

Do not change the signatures.

### Examples

```rust
let q = BoundedQueue::new(2);
q.push(1)    // Ok(())
q.push(2)    // Ok(()); the queue is full now, so another push would wait
q.len()      // 2
q.pop()      // Some(1)
q.push(3)    // Ok(())
q.close();
q.push(4)    // Err(4): the queue is closed
q.pop()      // Some(2): items already in the queue can still be popped
q.pop()      // Some(3)
q.pop()      // None: closed and empty
```

With a capacity of 2, the main thread pushes 100 to 104 while another thread pushes 200 to 204,
and two more threads pop until they get `None`. Once both have pushed everything, the main thread
closes the queue. Every push returns `Ok(())`, every item is popped exactly once, and each popping
thread gets the items of each pushing thread in the order they were pushed.
