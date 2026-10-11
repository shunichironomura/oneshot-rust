`#[derive]` does not look at the types of the fields. On a generic struct it puts a bound on every
type parameter, so `#[derive(Debug, Clone, Copy, PartialEq)]` generates
`impl<U: Copy> Copy for Length<U>`, `impl<U: PartialEq> PartialEq for Length<U>` and so on. `U`
only appears in `PhantomData<U>`, which is `Copy`, `PartialEq` and `Debug` for any `U`, but the
bounds are there anyway. Inside the generic `perimeter<U>` and `longest<U>` nothing is known about
`U`, so `Length<U>` is not `Copy`: `w + w` moves `w` and then uses it again (E0382), and `copied()`
requires `Length<U>: Copy` (E0277). Even `Length<Mm>` is neither `Copy` nor comparable, because
`Mm` implements nothing. This limitation of `#[derive]` is known as "imperfect derive".

The fix is to write the four impls by hand, without bounds on `U`:

```rust
impl<U> Clone for Length<U> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<U> Copy for Length<U> {}

impl<U> PartialEq for Length<U> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<U> fmt::Debug for Length<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Length").field(&self.value).finish()
    }
}
```

`Copy` has no methods, but it requires `Clone`. Since `Length<U>` is `Copy` for every `U`, `clone`
can simply return `*self`. `debug_tuple` prints `Length(...)` with the field formatted by `{:?}`, so
`10.0` appears as `10.0`, not `10` as with `{}`. With these impls, `perimeter`, `longest` and
`to_mm` compile unchanged.

Fixes that only make the editor code compile are not enough:

- Adding `U: Copy` to `perimeter` and `longest` (and deriving the traits on `Mm` and `Inch` so that
  the examples compile) changes the signatures, and `Length<Cubit>` in the tests is still neither
  `Copy`, comparable nor printable, so the tests do not compile.
- Avoiding the copies, for example with `Length::new(2.0 * (w.value + h.value))`, has the same
  problem: the tests themselves copy, compare and print lengths.
- Writing `Clone` by hand but keeping `#[derive(Copy)]` does not compile: `Length<U>` is then only
  `Copy` when `U: Copy`, so `*self` cannot move out of the reference.
- Changing the field to something like `PhantomData<fn() -> U>` does not help either, because the
  derive bounds the type parameters, not the fields.

### Alternatives

The impls can be written in other ways. `clone` can build a new length, `eq` can call `value()`
(which copies `*self`, since `Length<U>` is `Copy`), and `Debug` can use `write!`. Unlike
`debug_tuple`, `write!` ignores `{:#?}` (pretty printing), which the tests do not use.

```rust
impl<U> Clone for Length<U> {
    fn clone(&self) -> Self {
        Length::new(self.value)
    }
}

impl<U> Copy for Length<U> {}

impl<U> PartialEq for Length<U> {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl<U> fmt::Debug for Length<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Length({:?})", self.value)
    }
}
```
