`Box<dyn Unit>` needs `Unit` to be **dyn compatible** (formerly "object safe"): every method must be
callable through a vtable, without knowing the concrete type. Two methods break that.

- `promoted` returns `Self`. Behind `dyn Unit` the concrete type, and so the size of the return value,
  is unknown.
- `beats` is generic over `U`. A vtable has one entry per method, but a generic method is a
  whole family of functions, one per `U`.

Adding `where Self: Sized` to a method excludes it from the trait object: `dyn Unit` is not `Sized`,
so the method simply cannot be called on it, and the rest of the trait becomes dyn compatible.
`Knight` and `Archer` are `Sized`, so the harness can still call both methods on them. Fixing only
one of the two methods is not enough; the compiler checks the whole trait.

### Alternatives

`beats` does not need to be generic at all. Taking the other unit as `&dyn Unit` makes it
dyn compatible as it is, and it can even be called on `Box<dyn Unit>` values. Only `promoted` then
needs `where Self: Sized`.

```rust
pub trait Unit {
    fn name(&self) -> String;
    fn power(&self) -> u32;
    fn promoted(&self) -> Self
    where
        Self: Sized;

    fn beats(&self, other: &dyn Unit) -> bool {
        self.power() > other.power()
    }
}
```

`&Knight` and `&Archer` coerce to `&dyn Unit` at the call site, so callers do not change.
