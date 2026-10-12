Paths and visibility are both relative to modules, and the template makes five mistakes about them:

- `crate` is the root of the crate, and your file is not the root: it is the module `crate::user`
  (the root is the judge's `main.rs`). So `crate::units` does not exist. In `shapes`, `super` is
  `crate::user`, so write `super::units::Cm`, or the absolute path `crate::user::units::Cm`. The
  import in `checks` is already right: there `super` is `shapes`, so it goes up two levels with
  `super::super::units::Cm`.
- The field of a tuple struct is private unless it says otherwise: `pub struct Cm(u32)` makes the
  type public, but not the `u32` inside. Outside `units` you can neither build `Cm(width)`, nor read
  `.0`, nor match the pattern `Cm(w)`. `pub(super)` on the field makes it visible in `crate::user`
  and everything inside it, which includes `shapes` and `shapes::checks`, but not the harness.
- A plain `use` is a private import. `use checks::is_square;` lets `shapes` write `is_square`, but
  the path `crate::user::shapes::is_square` is private. `pub use` re-exports it.
- A re-export cannot be more visible than the item it re-exports: `pub use` of a `pub(super)`
  function is an error (E0364). Make `is_square` itself `pub`. Its module `checks` is still private,
  so `shapes::checks::is_square` stays closed and the re-export is the only public path. A private
  module with `pub` items, re-exported with `pub use`, is the usual way to choose the public paths
  of a library.
- `pub(super)` on `Rect::new` reaches `crate::user`, so `square` compiles, but the harness is
  outside `crate::user`. Make it `pub` (or `pub(crate)`, since the harness is in the same crate).

The pattern in `checks` reads `width` and `height`, private fields of `Rect`. That is fine: a
private field is visible in the module where its struct is defined and in all of that module's
descendants, and `checks` is inside `shapes`.

You cannot give `shapes` alone access to the field of `Cm`: `pub(in crate::user::shapes)` is
rejected, because a visibility can only be restricted to an ancestor module, and `shapes` is a
sibling of `units`. The closest you can get is their common parent, which is what `pub(super)`
(the same as `pub(in crate::user)`) means.

The errors do not show up all at once. The first compile reports the unresolved import, the pattern
on the private field of `Cm` in `checks`, the private import of `is_square` and the private `new`.
Only when `pub use` is written does the compiler check what may be re-exported. Fixing only the
reported errors takes two compiles.

### Alternatives

Absolute paths and `pub(crate)` work too. The harness is in the same crate, so `pub(crate)` is
enough, as long as the re-export is not wider than the function: `pub use` of a `pub(crate)`
function is also E0364.

```rust
pub mod shapes {
    use crate::user::units::Cm;

    pub(crate) use checks::is_square;

    pub struct Rect {
        width: Cm,
        height: Cm,
    }

    impl Rect {
        pub(crate) fn new(width: u32, height: u32) -> Rect {
            Rect { width: Cm(width), height: Cm(height) }
        }

        pub fn area(&self) -> u32 {
            self.width.0 * self.height.0
        }
    }

    mod checks {
        use super::Rect;
        use crate::user::units::Cm;

        pub(crate) fn is_square(rect: &Rect) -> bool {
            let Rect { width: Cm(w), height: Cm(h) } = rect;
            w == h
        }
    }
}

mod units {
    /// A length in whole centimeters.
    pub struct Cm(pub(crate) u32);
}

pub fn square(side: u32) -> shapes::Rect {
    shapes::Rect::new(side, side)
}

pub fn total_area(rects: &[shapes::Rect]) -> u32 {
    rects.iter().map(shapes::Rect::area).sum()
}
```

`checks` can also take `Cm` from its parent. A `use` declaration is an item too: the private import
in `shapes` is visible in its child `checks`, just like `Rect` is.

```rust
mod checks {
    use super::{Cm, Rect};

    pub fn is_square(rect: &Rect) -> bool {
        let Rect { width: Cm(w), height: Cm(h) } = rect;
        w == h
    }
}
```
