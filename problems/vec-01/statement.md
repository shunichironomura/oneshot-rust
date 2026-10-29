A to-do app keeps its jobs in a `Vec<Job>`, and every job expires after a number of ticks. Implement
the function `tick`, which updates the list in place. The struct `Job` is already in the editor.

```rust
#[derive(Debug, PartialEq)]
pub struct Job {
    pub id: u32,
    pub title: String,
    pub ttl: u32,
    pub done: bool,
}

pub fn tick(jobs: &mut Vec<Job>) -> Vec<u32>
```

- `tick` first decreases the `ttl` of every job by 1; a `ttl` of `0` stays `0`. Then it removes every
  job that is done (`done` is `true`) or whose `ttl` is now `0`, and returns the `id`s of the removed
  jobs in their original order. The remaining jobs keep their order and their decreased `ttl`.
- `jobs` may be empty, and `ttl` may be any `u32`, including `0` and `u32::MAX`. `tick` must not
  panic.

### Examples

A job is written here as `(id, title, ttl, done)`. On
`[(1, "a", 1, false), (2, "b", 3, false), (3, "c", 5, true)]`, `tick` returns `[1, 3]` and leaves
only `(2, "b", 2, false)`.
