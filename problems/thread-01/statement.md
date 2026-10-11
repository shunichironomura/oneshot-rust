The code below does not compile. **Without looking at the error message**, predict where the compiler
complains and fix it.

`parallel_sum` splits `values` into at most `workers` chunks, sums each chunk on its own thread, and
returns the total.

```rust
pub fn parallel_sum(values: &[u64], workers: usize) -> u64
```

Do not change the signature. `workers` is at least 1, and the work must still be split across
threads.

### Example

`parallel_sum(&[1, 2, 3, 4, 5], 2)` returns `15`.
