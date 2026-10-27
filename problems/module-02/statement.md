The code below does not compile. Fix it.

The code measures rectangles and is split into modules. `shapes` has the rectangle type `Rect`, and
its private child module `shapes::checks` has the function `is_square`. The private module `units`
has `Cm`, a tuple struct holding a length in centimeters. Your code is the module `crate::user`. The
test harness is another module of the same crate, outside `crate::user`, and it uses exactly these
paths:

```rust
crate::user::shapes::Rect::new(width: u32, height: u32) -> Rect
crate::user::shapes::Rect::area(&self) -> u32
crate::user::shapes::is_square(rect: &Rect) -> bool
crate::user::square(side: u32) -> Rect
crate::user::total_area(rects: &[Rect]) -> u32
```

- Lengths are whole centimeters, and `0` is allowed. `Rect::new(width, height)` creates a rectangle
  of that size, and `area` returns `width * height`.
- `is_square` returns `true` if the width and the height are equal. A `0` × `0` rectangle is a
  square.
- `square(side)` returns a `side` × `side` rectangle.
- `total_area` returns the sum of the areas of the rectangles in the slice (`0` for an empty slice).
- Areas and their sums always fit in a `u32`.
- Change only visibility, paths and `use` declarations. Keep every module, type and function where
  it is (do not move, rename or remove them).

### Examples

- `Rect::new(2, 3).area()` is `6`, and `square(4).area()` is `16`.
- `total_area(&[Rect::new(1, 2), Rect::new(3, 3)])` is `11`.
- `is_square(&square(2))` is `true`, and `is_square(&Rect::new(1, 2))` is `false`.
