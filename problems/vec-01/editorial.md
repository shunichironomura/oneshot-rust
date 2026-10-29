`Job` is not `Clone`, so a job can leave the list only by being moved out of it. Calling
`jobs.remove(i)` inside a loop over `jobs.iter()` is rejected, because the loop still borrows `jobs`
(and it would shift the indices under you anyway).

`Vec::extract_if` does exactly "take the matching elements out": it calls the predicate once for
each element of the range, in order, yields the elements for which it returns `true` **by value**,
and keeps the others in place and in order. Two details are easy to miss when writing it from
memory: the first argument is a range (`..` for the whole vector), and the iterator is lazy. If it
is dropped without being consumed, nothing is removed (the compiler warns about an unused
`ExtractIf`).

The predicate receives `&mut Job`, so `tick` can decrease the `ttl` and decide in the same pass. Use
`saturating_sub`: `job.ttl - 1` panics with an overflow when `ttl` is `0`, because debug builds (and
Miri) check arithmetic overflow.

### Alternatives

`retain_mut` gives the closure `&mut Job`, and the closure may push the removed ids to a vector
outside it.

```rust
pub fn tick(jobs: &mut Vec<Job>) -> Vec<u32> {
    let mut removed = Vec::new();
    jobs.retain_mut(|job| {
        job.ttl = job.ttl.saturating_sub(1);
        let keep = !job.done && job.ttl > 0;
        if !keep {
            removed.push(job.id);
        }
        keep
    });
    removed
}
```
