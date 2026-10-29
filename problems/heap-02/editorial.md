`BinaryHeap` is a **max-heap**: `pop` returns the greatest element. Wrapping each value in `Reverse`
flips the order, so the max-heap becomes a min-heap and pops the smallest value first. Negating the
values (`-x`) instead would overflow for `i32::MIN` (a panic in debug builds).

For `BinaryHeap<Reverse<i32>>`, repeated `pop` returns integers in ascending order, while
`into_sorted_vec` returns them in descending order (inside `Reverse`). `iter`, `into_iter` and
`into_vec` walk the heap's internal array, which is not sorted, so taking the first `k` of them gives
wrong answers.
Stopping when the heap runs out handles `k > v.len()`, whereas `Vec::with_capacity(k)` would panic
for `k = usize::MAX`.

### Alternatives

You can also keep just `k` values in a max-heap. The largest of the `k` smallest values seen so far
sits on top, and a smaller newcomer replaces it. For this `BinaryHeap<i32>` without `Reverse`,
`into_sorted_vec` returns the values in ascending order.

```rust
pub fn k_smallest(v: &[i32], k: usize) -> Vec<i32> {
    // Keep the k smallest values seen so far; the largest of them is on top.
    let mut heap = BinaryHeap::new();
    for &x in v {
        if heap.len() < k {
            heap.push(x);
        } else if heap.peek().is_some_and(|&top| x < top) {
            heap.pop();
            heap.push(x);
        }
    }
    heap.into_sorted_vec()
}
```

For a one-off query, sorting a copy is the simplest.

```rust
pub fn k_smallest(v: &[i32], k: usize) -> Vec<i32> {
    let mut sorted = v.to_vec();
    sorted.sort();
    sorted.truncate(k);
    sorted
}
```
