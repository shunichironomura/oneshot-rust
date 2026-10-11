A `static mut` is a global that any thread can read and write with no synchronization at all.
Edition 2024 denies creating a reference to one (`&NEXT`, `&mut NEXT`, or a method call that
borrows it), but reading and assigning it by value, as the template does, still compiles. When two
threads run `let id = NEXT; NEXT = id + 1;` at the same time, a write and a read of the same memory
are not ordered by anything. That is a **data race**, which is UB in Rust. In practice both threads
may read the same value and hand out the same id, but the program is UB even when that does not
happen, and Miri reports it as soon as two threads use `NEXT` without synchronization.

The fix needs no `unsafe`. An `AtomicU64` can be changed through a shared reference, so it can live
in a plain `static` (`AtomicU64::new` is a `const fn`). `fetch_add` adds to the value and returns
the **previous** value as one indivisible read-modify-write: no other thread can run between the
read and the write, so every call gets a different value. `reserve(n)` uses `fetch_add(n)` the same
way and claims the whole block `start..start + n` at once.

`Ordering::Relaxed` is enough. All modifications of one atomic variable happen in a single total
order (its modification order), and each `fetch_add` reads the value written just before it in that
order, which already makes the results unique. An operation never comes before an earlier operation
of the same thread in that order, so each thread sees increasing values. Stronger orderings matter
only when the counter is also used to pass other data between threads.

Pitfalls:

- `let start = NEXT.load(..); NEXT.store(start + n, ..);` is not a data race, since both operations
  are atomic, but another thread can run between them, and two threads then get overlapping ranges.
  Calling `next_id` `n` times inside `reserve` fails too: another thread can take an id in the
  middle, and that id ends up inside the returned range.
- `static mut NEXT: AtomicU64` with `unsafe { NEXT.fetch_add(..) }` does not compile in edition
  2024: the method call creates a shared reference to a `static mut`.
- `fetch_add` returns the value **before** the addition. Treating it as the new value gives ranges
  that overlap ids handed out earlier.

### Alternatives

A `Mutex` also makes the read-modify-write indivisible. `Mutex::new` is a `const fn`, so the mutex
can be a `static` too. Here `next_id` is written in terms of `reserve`. A lock costs more than an
atomic operation, but it also works for state that does not fit in one atomic.

```rust
use std::ops::Range;
use std::sync::Mutex;

static NEXT: Mutex<u64> = Mutex::new(0);

pub fn next_id() -> u64 {
    reserve(1).start
}

pub fn reserve(n: u64) -> Range<u64> {
    let mut next = NEXT.lock().unwrap();
    let start = *next;
    *next += n;
    start..start + n
}
```

`try_update` (stable since Rust 1.95; the older name `fetch_update` is deprecated) retries a closure
in a compare-and-swap loop until no other thread has changed the value in between. It helps when the
new value needs a check, for example to refuse to overflow.

```rust
pub fn reserve(n: u64) -> Range<u64> {
    let start = NEXT
        .try_update(Ordering::Relaxed, Ordering::Relaxed, |next| next.checked_add(n))
        .expect("ran out of ids");
    start..start + n
}
```
