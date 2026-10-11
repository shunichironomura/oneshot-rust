`Option::unwrap` takes the option **by value**. `self.state` sits behind `&mut self` (or `&self` in
`content`), and you cannot move a field out of a borrowed struct: that would leave `*self` with a
hole in it. So every `self.state.unwrap()` is error E0507, and so is `if let Some(s) = self.state`,
which moves the box into `s`.

How to fix each one depends on what the call needs:

- The transitions (`request_review`, `approve`, `reject`) call methods that take `self: Box<Self>`:
  they **consume** the old state and return the new one, so they need the box itself, not a
  reference to it. `Option::take` moves the value out and leaves `None` in its place, which is
  exactly why the field is an `Option`. Then store the new state.
- `content` and `add_text` only look at the state. `Option::as_ref` turns `&Option<Box<dyn State>>`
  into `Option<&Box<dyn State>>`, and the method call goes through the box to the state. In
  `add_text`, reading `self.state` while pushing onto `self.content` is fine because they are
  different fields, and the `&'a str` returned by `text_to_add` borrows `text`, not the state.

Following the compiler's suggestions blindly does not work for the transitions: `as_ref()` or
`as_mut()` only gives a reference, and calling a `self: Box<Self>` method through it is E0507
again, because it would move the box out from behind that reference. The suggested `ref s` in
`if let Some(ref s)` fails the same way. `clone()` is not available either: `Box<dyn State>` does
not implement `Clone`.

Using `take()` in `add_text` compiles, but if you forget to put the state back, the post is left
with `None` and stops working.

### Alternatives

The three transitions share one shape, so a helper can do the `take` once. `Option::map` applies
the transition only when there is a state, and `as_deref` gives an `Option<&dyn State>`.

```rust
pub fn add_text(&mut self, text: &str) {
    if let Some(state) = &self.state {
        self.content.push_str(state.text_to_add(text));
    }
}

pub fn content(&self) -> &str {
    self.state.as_deref().map_or("", |s| s.content(self))
}

pub fn request_review(&mut self) {
    self.transition(|s| s.request_review());
}

pub fn approve(&mut self) {
    self.transition(|s| s.approve());
}

pub fn reject(&mut self) {
    self.transition(|s| s.reject());
}

fn transition(&mut self, f: impl FnOnce(Box<dyn State>) -> Box<dyn State>) {
    self.state = self.state.take().map(f);
}
```

The trait methods themselves can be passed as functions: `State::approve` is a function from
`Box<dyn State>` to `Box<dyn State>`. `request_review` and `reject` are written the same way.

```rust
pub fn approve(&mut self) {
    self.transition(State::approve);
}

fn transition(&mut self, f: fn(Box<dyn State>) -> Box<dyn State>) {
    if let Some(s) = self.state.take() {
        self.state = Some(f(s));
    }
}
```

If you prefer to keep the original one-liners, `take().unwrap()` also works, since the state is
always put back:

```rust
pub fn approve(&mut self) {
    self.state = Some(self.state.take().unwrap().approve());
}
```
