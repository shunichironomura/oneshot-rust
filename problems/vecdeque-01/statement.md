A text editor remembers your actions so that you can undo them, but only the last `capacity` of
them. Implement this history, `History`, and the function `josephus`, which solves an old
counting-off puzzle.

```rust
pub struct History { /* up to you */ }

impl History {
    pub fn new(capacity: usize) -> History;
    pub fn record(&mut self, action: &str);
    pub fn undo(&mut self) -> Option<String>;
    pub fn recent(&self, n: usize) -> Vec<&str>;
    pub fn oldest(&self) -> Option<&str>;
    pub fn len(&self) -> usize;
}

pub fn josephus(n: usize, k: usize) -> Vec<usize>
```

- `new` returns an empty history that keeps at most `capacity` actions. `capacity` may be `0`.
- `record` adds `action` as the newest action. If the history then holds more than `capacity`
  actions, the oldest one is dropped, so a history with a capacity of `0` never keeps anything. A
  dropped action is gone for good.
- The same text may be recorded several times; each `record` is a separate action.
- `undo` removes the newest action and returns it. It returns `None` if the history is empty.
- `recent` returns the `n` newest actions, newest first. If fewer than `n` actions are kept, it
  returns all of them. `n` may be any `usize`, from `0` to `usize::MAX`.
- `oldest` returns the oldest kept action, or `None` if the history is empty.
- `len` returns the number of kept actions.

`josephus(n, k)` works as follows.

- `n` people, numbered `1` to `n`, stand in a circle in that order (person `1` comes after person
  `n`). Starting with person `1`, they count off `1`, `2`, `3`, … around the circle. The person
  who says `k` leaves the circle, and the next person still in it starts again from `1`. This
  repeats until everyone has left.
- It returns the numbers of the people in the order they leave.
- `n` may be `0`; the result is then empty. `k` is at least `1` and may be any `usize` up to
  `usize::MAX`, and the result must come quickly even for such a large `k`.

The fields of `History` are up to you: the one in the editor is only a starting point. You may use
anything from `std`, for example `std::collections::VecDeque`, a queue that can add and remove
elements at both ends.

### Examples

```rust
let mut h = History::new(3);
h.record("type");
h.record("bold");
h.record("paste");
h.record("delete");
// "type" was dropped: h.len() is 3 and h.oldest() is Some("bold").
// h.recent(2) is ["delete", "paste"], and h.recent(10) is ["delete", "paste", "bold"].
// h.undo() returns Some("delete"), and then h.recent(10) is ["paste", "bold"].
```

`josephus(7, 3)` returns `[3, 6, 2, 7, 5, 1, 4]`. After 3 and 6 have left, person 7 says `1`,
person 1 says `2` and person 2 says `3`, so person 2 leaves next.
