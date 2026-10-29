A to-do app keeps its jobs in a `Vec<Job>`, and the same job sometimes gets added twice. Implement
the function `dedup_titles`, which removes the duplicates in place. The struct `Job` is already in
the editor.

```rust
#[derive(Debug, PartialEq)]
pub struct Job {
    pub id: u32,
    pub title: String,
    pub ttl: u32,
    pub done: bool,
}

pub fn dedup_titles(jobs: &mut Vec<Job>) -> usize
```

- `dedup_titles` removes every job whose `title` equals the `title` of an earlier job in `jobs`,
  whether or not the two are next to each other: only the first job with each title is kept, and
  the other fields do not matter. Titles are compared exactly (`"Build"`, `"build"` and `"build "`
  are all different). The kept jobs keep their order. It returns the number of removed jobs.
- `jobs` may be empty.

### Examples

On jobs 1 to 5 with the titles `"a"`, `"b"`, `"a"`, `"c"`, `"b"`, `dedup_titles` returns `2` and
leaves jobs 1, 2 and 4.
