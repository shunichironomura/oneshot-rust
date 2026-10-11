In all four methods, `self` is a reference, so the fields are only borrowed. The template uses
`self.nickname` and `self.email` as values in three ways: as the scrutinee of a `match` whose pattern
`Some(nick)` binds by value, as the operand of `?`, and on the right of `let`. Each of them moves the
`Option<String>` out of the struct, and moving out of something behind a reference is an error
(E0507). `&mut self` does not change that: the struct belongs to the caller and must never be left
with a missing field, not even until the next line puts a new value back. So either look at the
`Option` through a reference, or swap a new value in and take the old one out in a single step.

- `display_name`: behind the move there is a second error. `nick` is a local variable, dropped when
  the function returns, so `&nick` cannot be returned (E0515). The result has to point into
  `self.nickname` itself. `as_deref()` turns `&Option<String>` into `Option<&str>` without moving
  anything, and `unwrap_or(&self.username)` supplies the fallback (`&String` coerces to `&str`).
- `email_domain`: `?` takes its operand by value, so `self.email?` moves as well. `&self.email?` does
  not help, because `?` binds tighter than `&`: it means `&(self.email?)`. The compiler's suggestion
  `self.email.clone()?` gets past the move, but then `email` is a local `String` and the returned
  slice would point into it (E0515 again). `self.email.as_deref()?` gives a `&str` that borrows from
  `self`, as the return type requires.
- `take_nickname` and `set_nickname`: `Option::take` puts `None` in place and returns the old value,
  and `Option::replace` puts `Some(value)` in place and returns the old value. Both move the old value
  out through `&mut` without ever leaving the field empty.

A habit from older editions is a trap here too: `match &self.nickname { Some(ref nick) => ... }` is
an error in edition 2024. When the scrutinee is a reference, the bindings are already references,
and writing `ref` there is no longer allowed. Write `match &self.nickname { Some(nick) => ... }` or
`match self.nickname { Some(ref nick) => ... }` instead.

### Alternatives

Matching on a reference works with nothing beyond `match`: `nick` and `email` are `&String`.

```rust
pub fn display_name(&self) -> &str {
    match &self.nickname {
        Some(nick) => nick,
        None => &self.username,
    }
}

pub fn email_domain(&self) -> Option<&str> {
    match &self.email {
        Some(email) => email.split_once('@').map(|(_, domain)| domain),
        None => None,
    }
}
```

`split_once` also returns an `Option`, so `email_domain` can use `?` twice.

```rust
pub fn email_domain(&self) -> Option<&str> {
    let (_, domain) = self.email.as_deref()?.split_once('@')?;
    Some(domain)
}
```

`std::mem::replace` does the same swap for any type, not only for `Option`, and `std::mem::take`
for any type with a default value (`None` for an `Option`).

```rust
pub fn take_nickname(&mut self) -> Option<String> {
    std::mem::take(&mut self.nickname)
}

pub fn set_nickname(&mut self, nick: &str) -> Option<String> {
    std::mem::replace(&mut self.nickname, Some(nick.to_string()))
}
```

Copying with `let old = self.nickname.clone();` and keeping the rest of the template also compiles
and passes, but it allocates a second `String` just to drop the original one line later. `take` and
`replace` hand over the existing `String` instead.
