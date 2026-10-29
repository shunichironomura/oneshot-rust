The code below does not compile. Fix it.

The test harness runs the async function below with its own small executor (only `std` is
available, there is no async runtime).

```rust
pub async fn sum_all(futs: Vec<Pin<Box<dyn Future<Output = u32>>>>) -> u32;
```

- `sum_all(futs)` awaits the futures one at a time, in the order of the vector: each future finishes
  before the next one is polled for the first time.
- It resolves to the sum of their outputs, or `0` for an empty vector. The sum always fits in a
  `u32`.
- Some of the futures the harness passes are not ready the first time they are polled (they return
  `Poll::Pending` once).

Do not change the signature.

### Examples

- Given three futures that resolve to `1`, `2` and `3`, `sum_all` resolves to `6`.
