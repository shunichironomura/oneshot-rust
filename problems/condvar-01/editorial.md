Everything a waiting thread looks at, the items and the closed flag, lives in one
`Mutex<State<T>>`. A `Condvar` adds the waiting: `wait` releases the lock and puts the thread to
sleep in one step, and takes the lock again before it returns. `wait_while(guard, condition)` wraps
this in a loop: it sleeps as long as `condition` is true and gives the guard back. The model
solution uses two condvars, one per kind of waiter: pops wait on `not_empty`, pushes on `not_full`.

The condition must be checked again after every wakeup. Being woken only means that something may
have changed. Another thread may take the item (or the room) before the woken thread gets the lock
back, and `Condvar` may even wake a thread for no reason at all (a spurious wakeup). With
`if ... { guard = cv.wait(guard).unwrap(); }`, such a pop returns `None` from an open queue, and
such a push adds an item to a full queue. The hidden cases make another thread take the item or
the room first. `wait_while`, or a `while` loop around `wait`, handles all of this.

Then every change must wake the threads that wait for it. A push wakes one waiting pop
(`not_empty.notify_one()`), and a pop wakes one waiting push. Closing changes the answer for
**every** waiting call, so `close` needs `notify_all` on both condvars. With `notify_one`, or no
notification at all, the other waiting threads sleep forever, and Miri reports "the evaluated
program deadlocked". If pushes and pops share a single condvar, `notify_one` may wake a
thread of the wrong kind (a pop, when only a push can go on), which goes back to sleep, and the
wakeup is lost. A hidden case makes this happen. Use `notify_all` everywhere in that case (see the
alternative).

A few more points:

- `wait` and `wait_while` take the guard by value and return it, because the lock is released
  while the thread sleeps. Assign the result, as in `state = cv.wait(state).unwrap()`.
- The closed flag must be inside the mutex. If it were an `AtomicBool` set without the lock,
  `close` could set it and notify between a pop's check and its sleep, and that pop would sleep
  forever. Under the lock, the check and the sleep happen as one step from `close`'s point of view.
- `Mutex` is not reentrant: calling `self.len()` from `push` while holding the guard deadlocks.
- `push` gives the item back in `Err(item)`. It still owns `item` there, because the item is only
  moved into the queue on success.
- `Mutex<State<T>>` is `Sync` when `T: Send`, and `Condvar` always is, so `BoundedQueue<T>` is
  `Sync` without any extra work, and the harness can share `&BoundedQueue<T>` between threads.
- The model solution notifies after dropping the guard, so that the woken thread does not have to
  wait for the lock right away. Notifying while holding the guard is also correct.

### Alternatives

One condvar with `notify_all` after every change, and explicit `while` loops instead of
`wait_while`. It is simpler, but each change wakes every waiting thread, and most of them go back to
sleep.

```rust
use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

pub struct BoundedQueue<T> {
    capacity: usize,
    state: Mutex<State<T>>,
    /// Notified after every change: push, pop or close.
    changed: Condvar,
}

struct State<T> {
    items: VecDeque<T>,
    closed: bool,
}

impl<T> BoundedQueue<T> {
    pub fn new(capacity: usize) -> Self {
        BoundedQueue {
            capacity,
            state: Mutex::new(State { items: VecDeque::new(), closed: false }),
            changed: Condvar::new(),
        }
    }

    pub fn push(&self, item: T) -> Result<(), T> {
        let mut state = self.state.lock().unwrap();
        while !state.closed && state.items.len() == self.capacity {
            state = self.changed.wait(state).unwrap();
        }
        if state.closed {
            return Err(item);
        }
        state.items.push_back(item);
        self.changed.notify_all();
        Ok(())
    }

    pub fn pop(&self) -> Option<T> {
        let mut state = self.state.lock().unwrap();
        while !state.closed && state.items.is_empty() {
            state = self.changed.wait(state).unwrap();
        }
        let item = state.items.pop_front();
        self.changed.notify_all();
        item
    }

    pub fn close(&self) {
        self.state.lock().unwrap().closed = true;
        self.changed.notify_all();
    }

    pub fn len(&self) -> usize {
        self.state.lock().unwrap().items.len()
    }
}
```
