With `type Item;`, an impl picks **one** item type for every call to `next`, and that type cannot
name the borrow of `&mut self` in a particular call. The template uses the lifetimes it has, `'s`
and `'t`, but the items do not live that long: a window is reborrowed from `self.slice` through
`&mut self`, and a line lives in `self.buf`, which the next call overwrites. rustc reports
"lifetime may not live long enough" at both `Some(...)`. The windows *cannot* be `&'s mut [T]`
either: neighboring windows overlap, so a caller could keep two `&mut` to the same element. This is
why slices in std have `chunks_mut` but no `windows_mut`.

A generic associated type (GAT) gives the item type a lifetime parameter, so each call can choose
it:

```rust
pub trait Lender {
    type Item<'a> where Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>>;
}
```

`Self::Item<'_>` uses the lifetime of `&mut self`, so an item keeps the lender mutably borrowed
until it is dropped. That is what makes these lenders safe: in `while let Some(w) = windows.next()`,
each `w` is gone before the next call, so two windows never exist at once. Each lender is borrowed
on its own, so the items of different lenders can be used together.

The `where Self: 'a` is required. `next` uses `Item<'a>` together with `&'a mut self`, which implies
`Self: 'a`, and rustc rejects a GAT that does not state that bound ("missing required bound on
`Item`"). The impls need it too: without it, rustc reports E0477 ("the type `Numbered<'t>` does not
fulfill the required lifetime"), even for `&'a str`. So the impls say

```rust
type Item<'a> = &'a mut [T] where Self: 'a;  // WindowsMut
type Item<'a> = &'a str where Self: 'a;      // Numbered
```

and `next` returns `Option<Self::Item<'_>>` in both.

A lifetime parameter on the trait (`trait Lender<'a>`) does not fit, because `count_all` is declared
with `L: Lender`. Generic code would also need `for<'a> Lender<'a>` everywhere.

### Alternatives

Name the lifetime of `next` explicitly, spell out the bounds that `Self: 'a` stands for, and write
the concrete item types in the impls.

```rust
pub trait Lender {
    type Item<'a> where Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

impl<'s, T> Lender for WindowsMut<'s, T> {
    type Item<'a> = &'a mut [T] where 's: 'a, T: 'a;

    fn next(&mut self) -> Option<&mut [T]> {
        let window = self.slice.get_mut(self.pos..self.pos + self.size)?;
        self.pos += 1;
        Some(window)
    }
}

impl<'t> Lender for Numbered<'t> {
    type Item<'a> = &'a str where 't: 'a;

    fn next(&mut self) -> Option<&str> {
        let line = self.lines.next()?;
        self.n += 1;
        self.buf.clear();
        write!(self.buf, "{}: {}", self.n, line).unwrap();
        Some(&self.buf)
    }
}
```
