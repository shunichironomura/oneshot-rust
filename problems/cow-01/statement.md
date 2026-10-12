Copying is expensive, so copy only when you actually have to change something. Implement three
small clone-on-write helpers. `Doc` is already in the editor.

```rust
use std::borrow::Cow;
use std::rc::Rc;

pub fn normalize_ws(s: &str) -> Cow<'_, str>;
pub fn clamp_all(v: &mut Cow<'_, [i32]>, max: i32);

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

`normalize_ws` cleans up the whitespace in `s`.

- Whitespace means the characters for which `char::is_ascii_whitespace` is true.
- The result is `s` with the whitespace at both ends removed and every run of whitespace between
  two non-whitespace characters replaced by a single `' '`.
- If no run has to be replaced (every run inside is already exactly one `' '`), the result is a part
  of `s` as it is: return it as `Cow::Borrowed`, a slice of `s`, without copying. Otherwise return
  `Cow::Owned`.

`clamp_all` limits the values in `v` to `max`.

- Every element greater than `max` becomes `max`. Other elements are kept.
- If no element is greater than `max`, `v` is left as it is: a `Cow::Borrowed` stays `Borrowed`,
  and nothing is copied.
- Otherwise a `Cow::Borrowed` becomes `Cow::Owned` with the clamped values. The borrowed slice
  itself does not change.
- A `Cow::Owned` stays `Owned`, and its `Vec` is changed in place (no new `Vec` is created).

`Doc` is a list of lines whose clones are cheap snapshots.

- `new` creates a `Doc` with new storage of its own, holding a copy of `lines`.
- `clone` (derived) makes a snapshot that shares the storage. Nothing is copied.
- `line(i)` returns line `i` (0-based), or `None` if `i` is out of range.
- `set_line(i, text)` replaces line `i` with `text`, and never changes any other `Doc`. If the
  storage is shared with another `Doc` that still exists, this `Doc` first gets a copy of its own.
  If it is not shared, the line is replaced in place, without copying.
- `shares_storage` tells whether two `Doc`s use the same storage (not just equal lines).

### Examples

```rust
normalize_ws("a b")       // Borrowed("a b")
normalize_ws("  hi\n")    // Borrowed("hi"), a slice of the input
normalize_ws("a  b\tc ")  // Owned("a b c")

let data = [1, 5, 3];
let mut v = Cow::Borrowed(&data[..]);
clamp_all(&mut v, 9);     // still Borrowed([1, 5, 3])
clamp_all(&mut v, 4);     // now Owned([1, 4, 3]); data is unchanged

let a = Doc::new(&["x", "y"]);
let mut b = a.clone();    // a.shares_storage(&b) is true
b.set_line(0, "z");       // now it is false
// a.line(0) == Some("x"), b.line(0) == Some("z")
```
