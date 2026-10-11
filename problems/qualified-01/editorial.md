Three of the functions make ambiguous calls, and the fourth compiles but calls the wrong method.

- `w.get()` in `summary`: `W` implements both widget traits, and both have a `get(&self)` method,
  so method call syntax cannot choose (E0034). Annotations like `let age: u8` do not help: Rust
  picks a method by its name and the receiver, never by the type the caller expects back. Name the
  trait in the path: `UsernameWidget::get(w)`. The argument `w: &W` tells the compiler that `Self`
  is `W`. `form.get()` in `next_age` is the same problem on a concrete type:
  `AgeWidget::get(form)`.
- `UsernameWidget::kind()` in `kinds`: `kind` has no `self` parameter, so nothing says which type's
  impl to use (E0790). Give the type too, with fully qualified syntax:
  `<W as UsernameWidget>::kind()`. `W::kind()` does not work either, because `W` has a `kind` from
  each trait (E0034 again).
- `form.label()` in `tag` compiles, but returns `"form of rustacean"`. When a type has an inherent
  method and a trait method with the same name and the same receiver (both take `&self` here),
  method call syntax picks the inherent one. `Labeled::label(form)` calls the trait's.

Do not follow the compiler's suggestions blindly here. For `kind`, it suggests
`<Form as UsernameWidget>::kind()`, or the same with the harness's type, but `kinds` must ask `W`,
whatever it is: the harness's type has a `kind` of its own. For `get`, it suggests
`UsernameWidget::get(&w)`. With only the trait in the path, `Self` is inferred from the type of the
argument: `&w` is a `&&W`, so `Self` would be `&W`, which implements neither trait. Pass `w` as it
is. (`<W as UsernameWidget>::get(&w)` would compile: once `Self` is fixed, `&&W` is coerced to
`&W`.)

`saturating_add` was already right: `age + 1` would panic on overflow when the age is `255`.

### Alternatives

Fully qualified syntax `<Type as Trait>::f(...)` works for the methods too, and states the type
explicitly everywhere.

```rust
pub fn summary<W: UsernameWidget + AgeWidget>(w: &W) -> String {
    format!("{} ({})", <W as UsernameWidget>::get(w), <W as AgeWidget>::get(w))
}

pub fn next_age(form: &Form) -> u8 {
    <Form as AgeWidget>::get(form).saturating_add(1)
}

pub fn tag(form: &Form) -> String {
    <Form as Labeled>::label(form)
}
```

Method call syntax is only ambiguous where both traits apply. Behind a generic parameter with a
single bound, `x.get()` and `x.label()` see only that trait (a type parameter has no inherent
methods), so small helpers make the calls unambiguous again.

```rust
fn username_of<U: UsernameWidget>(u: &U) -> String {
    u.get()
}

fn age_of<A: AgeWidget>(a: &A) -> u8 {
    a.get()
}

fn label_of<L: Labeled>(l: &L) -> String {
    l.label()
}

pub fn summary<W: UsernameWidget + AgeWidget>(w: &W) -> String {
    format!("{} ({})", username_of(w), age_of(w))
}

pub fn next_age(form: &Form) -> u8 {
    age_of(form).saturating_add(1)
}

pub fn tag(form: &Form) -> String {
    label_of(form)
}
```
