The code below does not compile. Fix it.

A sign-up form has two widgets, one for the username and one for the age. Each widget is a trait,
and both traits have an associated function named `kind` that names the widget. A type may override
the default `kind`. `Form` implements both traits (everything is already in the editor).

```rust
pub struct Form;

pub trait UsernameWidget {
    fn kind() -> &'static str { "username" }
}

pub trait AgeWidget {
    fn kind() -> &'static str { "age" }
}

pub fn kinds<W: UsernameWidget + AgeWidget>() -> (&'static str, &'static str)
```

`kinds::<W>()` returns the `kind` of the `UsernameWidget` impl of `W` and the `kind` of the
`AgeWidget` impl of `W`, in this order.

Do not change the struct, the two traits, their impls for `Form`, or the signature of `kinds`. The
test harness also implements both traits for a type of its own (and may override `kind` there) and
calls `kinds` with it.

### Examples

- `kinds::<Form>()` is `("username", "age")`.
