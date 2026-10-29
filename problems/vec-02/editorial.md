`dedup`, `dedup_by` and `dedup_by_key` remove only **consecutive** duplicates, so they leave
`[a, b, a, c, b]` unchanged. Remember the titles already seen in a `HashSet` instead:
`HashSet::insert` returns `false` for a title that is already there, which is exactly the job to
drop, and `retain` keeps the others in order.

But `jobs.retain(|job| seen.insert(job.title.as_str()))` does not compile: `retain` lends each
element to the closure only for the duration of that call, so the `&str` cannot be stored in
`seen`, which lives outside the closure ("borrowed data escapes outside of closure"). Storing owned
`String`s solves it.

### Alternatives

`extract_if` works too; `count` consumes the iterator.

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
