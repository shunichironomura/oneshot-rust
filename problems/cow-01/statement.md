Copying is expensive, so copy only when you actually have to change something. Implement
`clamp_all`, which limits the values in `v` to `max`.

```rust
use std::borrow::Cow;

pub fn clamp_all(v: &mut Cow<'_, [i32]>, max: i32);
```

- Every element greater than `max` becomes `max`. Other elements are kept.
- If no element is greater than `max`, `v` is left as it is: a `Cow::Borrowed` stays `Borrowed`,
  and nothing is copied.
- Otherwise a `Cow::Borrowed` becomes `Cow::Owned` with the clamped values. The borrowed slice
  itself does not change.
- A `Cow::Owned` stays `Owned`, and its `Vec` is changed in place (no new `Vec` is created).

### Examples

```rust
let data = [1, 5, 3];
let mut v = Cow::Borrowed(&data[..]);
clamp_all(&mut v, 9);     // still Borrowed([1, 5, 3])
clamp_all(&mut v, 4);     // now Owned([1, 4, 3]); data is unchanged
```
