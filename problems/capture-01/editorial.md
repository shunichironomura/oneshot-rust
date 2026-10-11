Since Rust 2021, a closure captures the **places** it uses, not whole variables. This closure only
uses `base.0`, so even though it is a `move` closure, it captures just that field: a `*mut u64`. A
closure is `Send` only if everything it captures is `Send`, and raw pointers are not, so
`thread::spawn` rejects it. The `SendPtr` wrapper with its `unsafe impl Send` is not captured at all.
In Rust 2018 the closure captured the whole variable `base`, and the same code compiled; the Edition
Guide uses exactly this pattern as its example.

The fix is to make the closure use the variable `base` itself. The model solution adds a method that
takes `self`: `base.get()` uses `base` as a whole, so the closure captures the whole `SendPtr`, which
is `Send`. Unlike a fix inside one closure, this keeps working in every closure that uses the
wrapper.

Some fixes look right but still capture only the field:

- `let SendPtr(p) = base;` inside the closure: destructuring uses only `base.0`.
- `SendPtr(base.0)` inside the closure: this builds a new wrapper from the captured field.

Keep taking the base pointer **once**, before spawning, as the template does. Calling
`out.as_mut_ptr()` in every iteration reborrows all of `out` mutably while the threads spawned
earlier are still writing through the previous pointer, and Miri reports undefined behavior (a data
race). Joining every thread before returning matters too: the pointer is only valid while `out` is
borrowed.

### Alternatives

Use the whole variable at the start of the closure. `let base = base;` copies the whole `SendPtr`
into the closure. `let _ = &base;` has the same effect; it is the line that `cargo fix --edition`
inserts when it migrates such code to Rust 2021.

```rust
handles.push(thread::spawn(move || {
    let base = base;
    for i in (w..len).step_by(workers) {
        // SAFETY: `i < len`, and no other thread writes to index `i`.
        unsafe { *base.0.add(i) = (i * i) as u64 };
    }
}));
```

Without `unsafe`, `thread::scope` lets the threads borrow `out`, and `chunks_mut` gives each thread
its own contiguous part. The chunk size must be at least 1, and an index within a chunk has to be
turned back into an index of `out`.

```rust
use std::thread;

pub fn fill_squares(out: &mut [u64], workers: usize) {
    // At least 1: `chunks_mut(0)` panics when `out` is empty.
    let chunk_size = out.len().div_ceil(workers).max(1);
    thread::scope(|s| {
        for (c, chunk) in out.chunks_mut(chunk_size).enumerate() {
            s.spawn(move || {
                let start = c * chunk_size;
                for (j, x) in chunk.iter_mut().enumerate() {
                    let i = start + j;
                    *x = (i * i) as u64;
                }
            });
        }
    });
}
```

The interleaved split works without `unsafe` too: deal out the `&mut u64` of each element to the
worker that owns it, then move each worker's share into a scoped thread.

```rust
use std::thread;

pub fn fill_squares(out: &mut [u64], workers: usize) {
    let mut shares: Vec<Vec<(usize, &mut u64)>> = (0..workers).map(|_| Vec::new()).collect();
    for (i, x) in out.iter_mut().enumerate() {
        shares[i % workers].push((i, x));
    }
    thread::scope(|s| {
        for share in shares {
            s.spawn(move || {
                for (i, x) in share {
                    *x = (i * i) as u64;
                }
            });
        }
    });
}
```

You can even keep `thread::spawn` if the threads only compute the squares and return them. The
calling thread writes them into `out` after joining.

```rust
use std::thread;

pub fn fill_squares(out: &mut [u64], workers: usize) {
    let len = out.len();
    let handles: Vec<_> = (0..workers)
        .map(|w| {
            thread::spawn(move || {
                (w..len).step_by(workers).map(|i| (i * i) as u64).collect::<Vec<_>>()
            })
        })
        .collect();
    for (w, handle) in handles.into_iter().enumerate() {
        let squares = handle.join().unwrap();
        for (x, square) in out.iter_mut().skip(w).step_by(workers).zip(squares) {
            *x = square;
        }
    }
}
```

Passing the address as a `usize` (`out.as_mut_ptr() as usize`, then `addr as *mut u64` in the
thread) also compiles and passes, because a `usize` is `Send`. But turning an integer back into a
pointer hides where the pointer came from, and tools like Miri can check such code less precisely. A
wrapper type says what you mean.
