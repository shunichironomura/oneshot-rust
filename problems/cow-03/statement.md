Copying is expensive, so copy only when you actually have to change something. Implement `Doc`, a
list of lines whose clones are cheap snapshots. `Doc` is already in the editor.

```rust
use std::rc::Rc;

#[derive(Clone)]
pub struct Doc {
    lines: Rc<Vec<String>>,
}

impl Doc {
    pub fn new(lines: &[&str]) -> Doc;
    pub fn line(&self, i: usize) -> Option<&str>;
    pub fn set_line(&mut self, i: usize, text: &str);
    pub fn shares_storage(&self, other: &Doc) -> bool;
}
```

- `new` creates a `Doc` with new storage of its own, holding a copy of `lines`.
- `clone` (derived) makes a snapshot that shares the storage. Nothing is copied.
- `line(i)` returns line `i` (0-based), or `None` if `i` is out of range.
- `set_line(i, text)` replaces line `i` with `text`, and never changes any other `Doc`. If the
  storage is shared with another `Doc` that still exists, this `Doc` first gets a copy of its own.
  If it is not shared, the line is replaced in place, without copying.
- `shares_storage` tells whether two `Doc`s use the same storage (not just equal lines).

### Examples

```rust
let a = Doc::new(&["x", "y"]);
let mut b = a.clone();    // a.shares_storage(&b) is true
b.set_line(0, "z");       // now it is false
// a.line(0) == Some("x"), b.line(0) == Some("z")
```
