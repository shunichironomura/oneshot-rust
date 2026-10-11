The `Outline` trait draws a frame of `*` around the text that a value prints with `Display`. The
trait and two types, `Point` and `Tags`, are already in the editor. Fill in the missing bodies.

```rust
pub trait Outline {
    fn outline(&self) -> String { /* to implement */ }
}

pub struct Point { pub x: i32, pub y: i32 }
pub struct Tags(pub Vec<String>);

impl fmt::Display for Point { /* to implement */ }
impl fmt::Display for Tags { /* to implement */ }

impl Outline for Point {}
impl Outline for Tags {}
```

- A `Point` displays as `(`, `x`, a comma, a space, `y` and `)`, with the coordinates in decimal:
  `(1, 3)`, `(-4, 0)`.
- A `Tags` displays as `[`, all its strings in order (even empty ones) separated by a comma and a
  space, and `]`: `[a, b, c]`. With no strings it displays as `[]`.
- Let `s` be the `Display` text of a value (it never contains a line break), and `w` the number of
  characters (`char`s) in `s`. `outline` returns these five lines joined by `\n`, with no `\n` at
  the end:
  1. `*` repeated `w + 4` times.
  2. `*`, then `w + 2` spaces, then `*`.
  3. `*`, a space, `s`, a space, then `*`.
  4. The same as line 2.
  5. The same as line 1.
- `outline` is a default method: the `impl Outline` blocks for `Point` and `Tags` stay empty. The
  test harness also has a type of its own that implements `Display`, and it writes an empty
  `impl Outline for ... {}` for it as well.
- Do not change the fields of `Point` and `Tags`, or the parameter (`&self`) and the return type
  (`String`) of `outline`. Anything else may be changed as needed.

### Examples

- `Point { x: 1, y: 3 }.to_string()` is `"(1, 3)"`.
- `Tags(vec!["red".to_string(), "green".to_string(), "blue".to_string()]).to_string()` is
  `"[red, green, blue]"`, and a `Tags` holding only `"solo"` displays as `[solo]`.
- `Point { x: 1, y: 3 }.outline()` returns these five lines:

```text
**********
*        *
* (1, 3) *
*        *
**********
```
