An emergency room does not serve patients in the order they arrive, but by how serious their
condition is. Implement the waiting list `Triage` and the function `k_smallest`.

```rust
pub struct Triage { /* up to you */ }

impl Triage {
    pub fn new() -> Self;
    pub fn arrive(&mut self, name: &str, severity: u8);
    pub fn next(&mut self) -> Option<String>;
    pub fn peek(&self) -> Option<&str>;
    pub fn len(&self) -> usize;
}

pub fn k_smallest(v: &[i32], k: usize) -> Vec<i32>
```

- `arrive` adds a patient to the waiting list. `severity` goes from `0` to `255`; a larger value is
  more urgent.
- The patient to serve next is the one with the highest severity. Among patients with the same
  severity, the one who arrived first is served first. Two patients may have the same name; they
  are still different patients.
- `next` removes the patient to serve next and returns their name. `peek` returns that name without
  removing the patient. Both return `None` when nobody is waiting.
- `len` returns the number of patients waiting.
- `k_smallest` returns the `k` smallest values of `v` in ascending order, keeping duplicates. `v` may
  hold any `i32` values (including `i32::MIN` and `i32::MAX`), and `k` may be any `usize`. If `k` is
  `v.len()` or more, it returns all the values of `v`, sorted.

The fields of `Triage` are up to you: the one in the editor is only a starting point. You may use
anything from `std`, for example `std::collections::BinaryHeap`.

### Examples

```rust
let mut t = Triage::new();
t.arrive("ann", 2);
t.arrive("bob", 5);
t.arrive("cid", 5);
t.arrive("dan", 1);
// t.peek() is Some("bob") and t.len() is 4.
// Calling t.next() five times gives Some("bob"), Some("cid"), Some("ann"), Some("dan"), None.
```

`k_smallest(&[5, 1, 4, 1, 3], 3)` returns `[1, 1, 3]`.
