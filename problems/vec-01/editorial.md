`Job` is not `Clone`, so a job can leave the list only by being moved out of it. A loop over
`jobs.iter()` sees `&Job`, and `out.push(*job)` is rejected ("cannot move out of `*job` which is
behind a shared reference"). Calling `jobs.remove(i)` inside that loop is rejected too, because the
loop still borrows `jobs` (and it would shift the indices under you anyway).

`Vec::extract_if` does exactly "take the matching elements out": it calls the predicate once for
each element of the range, in order, yields the elements for which it returns `true` **by value**,
and keeps the others in place and in order. Two details are easy to miss when writing it from
memory: the first argument is a range (`..` for the whole vector), and the iterator is lazy. If it
is dropped without being consumed, nothing is removed (the compiler warns about an unused
`ExtractIf`).

The predicate receives `&mut Job`, so `tick` can decrease the `ttl` and decide in the same pass. Use
`saturating_sub`: `job.ttl - 1` panics with an overflow when `ttl` is `0`, because debug builds (and
Miri) check arithmetic overflow.

`dedup`, `dedup_by` and `dedup_by_key` remove only **consecutive** duplicates, so they leave
`[a, b, a, c, b]` unchanged. Remember the titles already seen in a `HashSet` instead. But
`jobs.retain(|job| seen.insert(job.title.as_str()))` does not compile: `retain` lends each element
to the closure only for the duration of that call, so the `&str` cannot be stored in `seen`, which
lives outside the closure ("borrowed data escapes outside of closure"). Storing owned `String`s
solves it.

In `move_to_front`, `jobs.swap(0, i)` would break the order of the other jobs. Rotating the prefix
`jobs[..=i]` right by one moves job `i` to the front and shifts jobs `0..i` back by one place.

### Alternatives

Without `extract_if`, take the whole vector out with `std::mem::take` (leaving an empty one behind),
split it with `partition`, and put the rest back.

```rust
pub fn take_done(jobs: &mut Vec<Job>) -> Vec<Job> {
    let (done, rest) = std::mem::take(jobs).into_iter().partition(|job| job.done);
    *jobs = rest;
    done
}
```

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

`extract_if` also works for `dedup_titles`; `count` consumes the iterator.

```rust
pub fn dedup_titles(jobs: &mut Vec<Job>) -> usize {
    let mut seen = HashSet::new();
    jobs.extract_if(.., |job| !seen.insert(job.title.clone())).count()
}
```

To avoid cloning the titles, decide first which jobs to keep while `seen` borrows the titles, then
remove the others. The borrow ends before `retain` starts, and `retain` visits the elements in order,
exactly once.

```rust
pub fn dedup_titles(jobs: &mut Vec<Job>) -> usize {
    let mut seen = HashSet::new();
    let keep: Vec<bool> = jobs.iter().map(|job| seen.insert(job.title.as_str())).collect();
    let mut keep = keep.into_iter();
    let before = jobs.len();
    jobs.retain(|_| keep.next().unwrap());
    before - jobs.len()
}
```

`move_to_front` can also remove the job and insert it again at index 0.

```rust
pub fn move_to_front(jobs: &mut Vec<Job>, id: u32) -> bool {
    let Some(i) = jobs.iter().position(|job| job.id == id) else {
        return false;
    };
    let job = jobs.remove(i);
    jobs.insert(0, job);
    true
}
```
