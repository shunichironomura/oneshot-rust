`thread::spawn` requires its closure to be `Send`, and a closure is `Send` only if everything it
captures is. The compiler reports these problems one at a time, so fixing only the first one is not
enough:

1. `Rc` is not `Send`. Its reference count is updated without synchronization, so two threads
   cloning or dropping handles at the same time could corrupt it. `Arc` updates the count atomically.
2. `Arc<T>` is `Send` only if `T` is `Send + Sync`, because every thread holding a handle reaches the
   same `T` through a shared reference. `RefCell` and `Cell` are not `Sync`: they change their
   contents through `&` without synchronization. So `Arc<RefCell<_>>` and `Arc<Cell<_>>` are rejected
   too. Their thread-safe counterparts are `Mutex` (or `RwLock`) and the atomic types such as
   `AtomicUsize`.
3. `Mutex::into_inner` returns a `LockResult`, not the value itself, because the mutex is
   *poisoned* if a thread panicked while holding the lock. That needs one more `unwrap()`.

`words` is only read, so `Arc<Vec<String>>` is enough; it needs no lock. `Arc::try_unwrap` succeeds
only when no other handle exists. The clones moved into a thread are dropped when its closure
finishes, so after all the `join`s the original handle is the last one. `Ordering::Relaxed` is enough
for the counter: no increment is lost, and `join` makes everything a thread did visible to the thread
that joins it.

### Alternatives

One `Mutex` can protect the map and the counter together, which also takes the lock once per word
instead of once per char. `Arc::into_inner` is like `Arc::try_unwrap`, but returns `None` when other
handles still exist.

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn letter_counts(words: Vec<String>, workers: usize) -> (HashMap<char, usize>, usize) {
    let words = Arc::new(words);
    let state = Arc::new(Mutex::new((HashMap::new(), 0)));

    let mut handles = Vec::new();
    for w in 0..workers {
        let words = Arc::clone(&words);
        let state = Arc::clone(&state);
        handles.push(thread::spawn(move || {
            for word in words.iter().skip(w).step_by(workers) {
                let mut guard = state.lock().unwrap();
                let (counts, processed) = &mut *guard;
                for c in word.chars() {
                    *counts.entry(c).or_insert(0) += 1;
                }
                *processed += 1;
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }

    Arc::into_inner(state).unwrap().into_inner().unwrap()
}
```

Instead of unwrapping the `Arc`, you can lock the mutex one last time and move the map out with
`std::mem::take`, which leaves an empty map behind. Here the counter is a `Mutex<usize>` as well.

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn letter_counts(words: Vec<String>, workers: usize) -> (HashMap<char, usize>, usize) {
    let words = Arc::new(words);
    let counts = Arc::new(Mutex::new(HashMap::new()));
    let processed = Arc::new(Mutex::new(0));

    let mut handles = Vec::new();
    for w in 0..workers {
        let words = Arc::clone(&words);
        let counts = Arc::clone(&counts);
        let processed = Arc::clone(&processed);
        handles.push(thread::spawn(move || {
            for word in words.iter().skip(w).step_by(workers) {
                for c in word.chars() {
                    *counts.lock().unwrap().entry(c).or_insert(0) += 1;
                }
                *processed.lock().unwrap() += 1;
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }

    let counts = std::mem::take(&mut *counts.lock().unwrap());
    (counts, *processed.lock().unwrap())
}
```

Without the shared map, each thread could also count into a local `HashMap` and return it through
its `JoinHandle`, and the calling thread would merge the results with no `Mutex` at all. With
`thread::scope` instead of `thread::spawn`, the closures could borrow `words`, a `Mutex` and an
`AtomicUsize` without `Arc`, but `RefCell` and `Cell` would still be rejected: `&T` is `Send` only
if `T` is `Sync`.
