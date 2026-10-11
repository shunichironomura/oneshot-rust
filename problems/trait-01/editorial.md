`area` and `name` are required methods, so each `impl Shape for ...` block supplies its own version.
`describe` has a body in the trait itself: a **default method**. It is written once in terms of
`self.name()` and `self.area()`, and every implementor, including ones defined elsewhere, gets it for
free. `{:.2}` in `format!` rounds a float to two decimal places.

`&dyn Shape` is a reference to any type that implements `Shape`; the call `shape.area()` is
dispatched to the right implementation at run time. That is what lets one slice hold a `Rect` and a
`Circle` together. Summing an iterator of `f64` with `sum()` gives `0.0` for an empty slice.

### Alternatives

A plain loop works just as well for `total_area`.

```rust
pub fn total_area(shapes: &[&dyn Shape]) -> f64 {
    let mut total = 0.0;
    for shape in shapes {
        total += shape.area();
    }
    total
}
```

`name` could also return `&'static str`, but the signature is given here, so `.to_string()` or
`String::from` is needed to build the `String`.
