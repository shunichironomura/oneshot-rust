A to-do app keeps its jobs in a `Vec<Job>`. Implement four functions that update the list in place.
The struct `Job` is already in the editor.

```rust
#[derive(Debug, PartialEq)]
pub struct Job {
    pub id: u32,
    pub title: String,
    pub ttl: u32,
    pub done: bool,
}

pub fn take_done(jobs: &mut Vec<Job>) -> Vec<Job>
pub fn tick(jobs: &mut Vec<Job>) -> Vec<u32>
pub fn dedup_titles(jobs: &mut Vec<Job>) -> usize
pub fn move_to_front(jobs: &mut Vec<Job>, id: u32) -> bool
```

- `take_done` removes every job whose `done` is `true` from `jobs` and returns those jobs. Both the
  returned jobs and the jobs left in `jobs` keep their original relative order.
- `tick` first decreases the `ttl` of every job by 1; a `ttl` of `0` stays `0`. Then it removes every
  job that is done (`done` is `true`) or whose `ttl` is now `0`, and returns the `id`s of the removed
  jobs in their original order. The remaining jobs keep their order and their decreased `ttl`.
- `dedup_titles` removes every job whose `title` equals the `title` of an earlier job in `jobs`,
  whether or not the two are next to each other: only the first job with each title is kept, and
  the other fields do not matter. Titles are compared exactly (`"Build"`, `"build"` and `"build "`
  are all different). The kept jobs keep their order. It returns the number of removed jobs.
- `move_to_front` moves the first job whose `id` field equals the argument `id` to index 0, leaving
  the other jobs in their original order, and returns `true`. If no job has that `id`, it returns
  `false` and leaves `jobs` unchanged. Several jobs may share an `id`.
- `jobs` may be empty, and `ttl` may be any `u32`, including `0` and `u32::MAX`. No function may
  panic.

### Examples

A job is written here as `(id, title, ttl, done)`.

- On `[(1, "a", 5, false), (2, "b", 5, true), (3, "c", 5, false), (4, "d", 5, true)]`, `take_done`
  returns jobs 2 and 4, in that order, and leaves jobs 1 and 3.
- On `[(1, "a", 1, false), (2, "b", 3, false), (3, "c", 5, true)]`, `tick` returns `[1, 3]` and
  leaves only `(2, "b", 2, false)`.
- On jobs 1 to 5 with the titles `"a"`, `"b"`, `"a"`, `"c"`, `"b"`, `dedup_titles` returns `2` and
  leaves jobs 1, 2 and 4.
- On jobs with the ids `[1, 2, 3, 4]`, `move_to_front(&mut jobs, 3)` returns `true` and leaves the
  ids `[3, 1, 2, 4]`.
