`thread::spawn` requires its closure to be `'static`: the new thread may outlive the function, so it
must not borrow anything local. Each `chunk` borrows from `values`, which is only guaranteed to live
until `parallel_sum` returns, so the compiler rejects the code even though every thread is joined
before the return.

`thread::scope` fixes exactly this. All threads spawned on the scope `s` are joined before `scope`
returns, so their closures may borrow `values`. The value returned by the closure passed to `scope`
becomes the return value of `scope`.

### Alternatives

Copy each chunk into an owned `Vec` so that the thread no longer borrows anything. This works with
`thread::spawn`, at the cost of copying the input.

```rust
use std::thread;

pub fn parallel_sum(values: &[u64], workers: usize) -> u64 {
    let chunk_size = values.len().div_ceil(workers).max(1);
    let handles: Vec<_> = values
        .chunks(chunk_size)
        .map(|chunk| {
            let chunk = chunk.to_vec();
            thread::spawn(move || chunk.iter().sum::<u64>())
        })
        .collect();
    handles.into_iter().map(|h| h.join().unwrap()).sum()
}
```

Wrapping `values` in an `Arc` does not help by itself here, because `values` is a borrowed slice; you
would have to copy it into an `Arc<[u64]>` first.
