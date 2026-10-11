The code below does not compile. **Without looking at the error message**, predict where the compiler
complains and fix it.

A sign-up form has two widgets, one for the username and one for the age. Each widget is a trait,
and both traits name their methods `get` and `kind`. `Form` implements both of them and a third
trait, `Labeled`, and it also has an inherent method named `label` (everything is already in the
editor).

```rust
pub struct Form {
    pub username: String,
    pub age: u8,
}

pub trait UsernameWidget {
    fn get(&self) -> String;
    fn kind() -> &'static str { "username" }
}

pub trait AgeWidget {
    fn get(&self) -> u8;
    fn kind() -> &'static str { "age" }
}

pub trait Labeled {
    fn label(&self) -> String; // for Form: "user:<username>"
}

impl Form {
    pub fn label(&self) -> String; // "form of <username>"
}

pub fn summary<W: UsernameWidget + AgeWidget>(w: &W) -> String
pub fn kinds<W: UsernameWidget + AgeWidget>() -> (&'static str, &'static str)
pub fn next_age(form: &Form) -> u8
pub fn tag(form: &Form) -> String
```

- `summary(w)` returns the two widget values of `w`: the result of `get` from `UsernameWidget`, a
  space, and the result of `get` from `AgeWidget` in parentheses, for example `"rustacean (28)"`.
- `kinds::<W>()` returns the `kind` of the `UsernameWidget` impl of `W` and the `kind` of the
  `AgeWidget` impl of `W`, in this order. For `Form`, it is `("username", "age")`.
- `next_age(form)` returns the age of the form plus 1. If the age is already `255`, it stays `255`.
- `tag(form)` returns the label that the `Labeled` impl gives the form, `"user:<username>"`.

Do not change the struct, the three traits, their impls for `Form`, the inherent method
`Form::label`, or the signatures of the four functions. The test harness also implements both
widget traits for a type of its own (with its own `get` methods, and it may override `kind` too)
and calls `summary` and `kinds` with it.

### Examples

With `f = Form { username: "rustacean".to_string(), age: 28 }`:

- `summary(&f)` is `"rustacean (28)"`.
- `kinds::<Form>()` is `("username", "age")`.
- `next_age(&f)` is `29`, and `tag(&f)` is `"user:rustacean"`.
