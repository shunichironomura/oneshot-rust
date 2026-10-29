Packets whose payloads have different types can share a type only if the payload's type is erased:
`Packet<dyn Display>`, a **custom dynamically sized type** (DST) whose last field is a trait object.
Like `dyn Display` itself, it can only be used behind a pointer, and a pointer to it, such as
`Box<Packet<dyn Display>>`, is a fat pointer that stores the payload's vtable next to the address.
Every type parameter is `Sized` by default, so the struct needs `T: ?Sized` before
`Packet<dyn Display>` can even be written. Only the last field may be unsized, and `payload` is the
last one.

A `Packet<dyn Display>` cannot be built directly. Instead, it is made from a sized packet by an
**unsizing coercion**: behind a pointer, `Packet<T>` coerces to `Packet<dyn Display>` when `T`
implements `Display`, because the type parameter appears only in the last field. That is why the
bodies work unchanged:

- `label` builds a `Box<Packet<T>>`, and the return type `Box<Packet<dyn Display>>` coerces it.
  With `-> Box<Packet<T>>`, `label` itself compiles, but `label(1, 5)` and `label(2, "x")` then
  have different types, so `vec![label(1, 5), label(2, "x")]` does not compile.
- In a signature, `Box<Packet<dyn Display>>` means `Box<Packet<dyn Display + 'static>>`, so
  `T: Display` alone is rejected with "the parameter type `T` may not live long enough" (E0310).
  `T: Display + 'static` fixes it: payloads such as `5`, `"x"` or a `String` borrow nothing
  short-lived.
- `describe` takes `&[Box<Packet<dyn Display>>]`, a slice of the boxes that the `Vec` holds. The
  elements of a slice must be sized, so `&[Packet<dyn Display>]` is not a valid type. `{}` formats
  `&p.payload` through the vtable of each payload.

### Alternatives

`label` can keep the lifetime of its payload instead of requiring `'static`. To accept such packets,
`describe` then needs a lifetime too.

```rust
pub fn label<'a, T: Display + 'a>(id: u32, payload: T) -> Box<Packet<dyn Display + 'a>> {
    Box::new(Packet { id, payload })
}

pub fn describe(ps: &[Box<Packet<dyn Display + '_>>]) -> Vec<String> {
    ps.iter().map(|p| format!("#{}: {}", p.id, &p.payload)).collect()
}
```

`describe` can also be generic. It needs `T: ?Sized`, because the harness calls it with
`T = dyn Display`.

```rust
pub fn describe<T: Display + ?Sized>(ps: &[Box<Packet<T>>]) -> Vec<String> {
    ps.iter().map(|p| format!("#{}: {}", p.id, &p.payload)).collect()
}
```
