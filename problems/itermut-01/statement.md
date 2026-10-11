Write an iterator over the **runs** of a mutable slice: the groups of consecutive equal elements.
Each run is handed out as a mutable subslice, so the caller can write through it, and runs can be
taken from either end.

```rust
pub struct RunsMut<'a, T> {
    rest: &'a mut [T],
}

pub fn runs_mut<T: PartialEq>(v: &mut [T]) -> RunsMut<'_, T>

impl<'a, T: PartialEq> Iterator for RunsMut<'a, T> {
    type Item = &'a mut [T];
    fn next(&mut self) -> Option<&'a mut [T]>;
}

impl<'a, T: PartialEq> DoubleEndedIterator for RunsMut<'a, T> {
    fn next_back(&mut self) -> Option<&'a mut [T]>;
}
```

`RunsMut` is already in the editor. Its field is private, so you may change it if you like.

- `v` is split into runs, each as long as possible: a new run starts at every element that is not
  equal to the element just before it. Every run has at least one element, and an empty `v` has
  no runs. Equal values that are not next to each other belong to different runs.
- Elements are compared with `==`. `T` is only required to be `PartialEq`, so a value need not be
  equal to itself: `f64::NAN == f64::NAN` is `false`, and every NaN is a run of its own.
- `next` yields the runs from the front (left to right), and `next_back` from the back (right to
  left). Calls to both can be mixed: every run is yielded exactly once, whichever end it comes
  from. Once all runs have been yielded, both return `None`, and keep doing so.
- Each run is yielded as the subslice of `v` that it occupies. The caller may keep all the yielded
  runs and read and write through them at the same time, also after the iterator is gone. The
  writes go to `v`.
- Nothing may panic, and `unsafe` code is not allowed.

### Examples

```rust
let mut v = [1, 1, 2, 3, 3, 3];
let runs: Vec<&mut [i32]> = runs_mut(&mut v).collect(); // [[1, 1], [2], [3, 3, 3]]
for run in runs {
    let len = run.len() as i32;
    run.fill(len);
}
// v is now [2, 2, 1, 3, 3, 3]

let mut v = [1, 1, 2, 3, 3, 3];
runs_mut(&mut v).rev().collect::<Vec<_>>() // [[3, 3, 3], [2], [1, 1]]

let mut it = runs_mut(&mut v);
it.next()      // Some([1, 1])
it.next_back() // Some([3, 3, 3])
it.next()      // Some([2])
it.next()      // None
it.next_back() // None
```
