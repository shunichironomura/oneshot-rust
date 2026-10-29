`UsernameWidget::kind()` names the trait, but `kind` has no `self` parameter, so nothing says which
type's impl to use (E0790). With a method, the type of the receiver fills that gap; with an
associated function, you have to give the type yourself, with fully qualified syntax:
`<W as UsernameWidget>::kind()`. `W::kind()` does not work either, because `W` has a `kind` from
each trait (E0034).

Do not follow the compiler's suggestion blindly here. It suggests `<Form as UsernameWidget>::kind()`,
or the same with the harness's type, but `kinds` must ask `W`, whatever it is: the harness's type has
a `kind` of its own.

### Alternatives

A helper generic over a single trait makes the path unambiguous: behind a type parameter with one
bound, `U::kind()` can only mean that trait's `kind`.

```rust
fn username_kind<U: UsernameWidget>() -> &'static str {
    U::kind()
}

fn age_kind<A: AgeWidget>() -> &'static str {
    A::kind()
}

pub fn kinds<W: UsernameWidget + AgeWidget>() -> (&'static str, &'static str) {
    (username_kind::<W>(), age_kind::<W>())
}
```
