`W` implements both widget traits, and both have a `get(&self)` method, so method call syntax cannot
choose (E0034). Annotations like `let age: u8` do not help: Rust picks a method by its name and the
receiver, never by the type the caller expects back. Name the trait in the path:
`UsernameWidget::get(w)`. The argument `w: &W` tells the compiler that `Self` is `W`.

Do not follow the compiler's suggestion blindly here. It suggests `UsernameWidget::get(&w)`. With
only the trait in the path, `Self` is inferred from the type of the argument, and a path call does
no auto-ref or auto-deref the way method call syntax does: `&w` is a `&&W`, so `Self` would be `&W`,
which implements neither trait. Pass `w` as it is. (`<W as UsernameWidget>::get(&w)` would compile:
once `Self` is fixed, `&&W` is coerced to `&W`.)

### Alternatives

Fully qualified syntax `<Type as Trait>::f(...)` states the type explicitly.

```rust
pub fn summary<W: UsernameWidget + AgeWidget>(w: &W) -> String {
    format!("{} ({})", <W as UsernameWidget>::get(w), <W as AgeWidget>::get(w))
}
```

Method call syntax is only ambiguous where both traits apply. Behind a generic parameter with a
single bound, `x.get()` sees only that trait, so small helpers make the calls unambiguous again.

```rust
fn username_of<U: UsernameWidget>(u: &U) -> String {
    u.get()
}

fn age_of<A: AgeWidget>(a: &A) -> u8 {
    a.get()
}

pub fn summary<W: UsernameWidget + AgeWidget>(w: &W) -> String {
    format!("{} ({})", username_of(w), age_of(w))
}
```
