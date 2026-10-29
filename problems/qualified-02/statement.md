The code below does not compile. Fix it.

A sign-up form has two widgets, one for the username and one for the age. Each widget is a trait,
and both traits name their method `get`. `Form` implements both of them (everything is already in
the editor).

```rust
pub struct Form {
    pub username: String,
    pub age: u8,
}

pub trait UsernameWidget {
    fn get(&self) -> String;
}

pub trait AgeWidget {
    fn get(&self) -> u8;
}

pub fn summary<W: UsernameWidget + AgeWidget>(w: &W) -> String
```

`summary(w)` returns the two widget values of `w`: the result of `get` from `UsernameWidget`, a
space, and the result of `get` from `AgeWidget` in parentheses.

Do not change the struct, the two traits, their impls for `Form`, or the signature of `summary`. The
test harness also implements both traits for a type of its own (with its own `get` methods) and
calls `summary` with it.

### Examples

- `summary(&Form { username: "rustacean".to_string(), age: 28 })` is `"rustacean (28)"`.
