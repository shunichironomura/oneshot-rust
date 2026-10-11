`Post` itself is fine: `take()` moves the box out of the `Option` and leaves `None` until the next
state is stored, and `add_text` and `content` only borrow the state. The errors are the three calls
`s.request_review()`, `s.approve()` and `s.reject()`, on an `s` of type `Box<dyn State>`.

These methods take `self` **by value**. Method lookup goes through the box to the `dyn State`
inside, so the call would have to move that state out of the box and pass it as an argument. But
the size of a `dyn State` is only known at run time (it may be a `Draft` or a `PendingReview`), and
a value whose size is not known statically cannot be moved. That is E0161 ("cannot move a value of
type `dyn State`"). The trait itself is accepted, and the methods can be called on a `Draft`, just
not through the trait object.

The receiver `self: Box<Self>` makes the method take the box itself, which has a known size. It
can be called on a `Box<dyn State>`, and it still consumes the old state, which is what a
transition needs. Each implementation then receives its own concrete box, such as `Box<Draft>`.
`Box::new(self)` would now put that box into another box, and `Box<Draft>` does not implement
`State` (E0277). Return `self` as it is: `Box<Draft>` coerces to `Box<dyn State>` (an unsizing
coercion), and the old allocation is reused. `PendingReview::approve` takes `mut self: Box<Self>`
and increments `approvals` through the box.

Two tempting fixes do not work:

- `where Self: Sized` on the transitions. It is the usual cure for a method that makes a trait
  unusable as `dyn Trait` (one that returns `Self`, or a generic one), but this trait is already
  usable. It only removes the methods from `dyn State`, and the calls fail with "the `approve`
  method cannot be invoked on a trait object".
- `(*s).approve()`. That spells out the same move out of the box, and fails the same way.

### Alternatives

The transitions can also take `&self` and build the next state. That works, but every transition
allocates a new box, even when the state does not change, and `PendingReview` has to copy its
approvals into the new value. `Post` stays as in the template.

```rust
trait State {
    fn request_review(&self) -> Box<dyn State>;
    fn approve(&self) -> Box<dyn State>;
    fn reject(&self) -> Box<dyn State>;

    // `text_to_add` and `content` as before.
}

impl State for PendingReview {
    fn request_review(&self) -> Box<dyn State> {
        Box::new(PendingReview { approvals: self.approvals })
    }

    fn approve(&self) -> Box<dyn State> {
        if self.approvals + 1 >= 2 {
            Box::new(Published)
        } else {
            Box::new(PendingReview { approvals: self.approvals + 1 })
        }
    }

    fn reject(&self) -> Box<dyn State> {
        Box::new(Draft)
    }
}

// `Draft` and `Published` return `Box::new(Draft)` and `Box::new(Published)` where they do not
// change the state.
```

With `self: Box<Self>`, the three transitions of `Post` share one shape, so a helper can do the
`take` once. Here `State::approve` means `<dyn State as State>::approve`, a function from
`Box<dyn State>` to `Box<dyn State>`, and `Option::map` applies it only when there is a state.

```rust
pub fn request_review(&mut self) {
    self.transition(State::request_review);
}

pub fn approve(&mut self) {
    self.transition(State::approve);
}

pub fn reject(&mut self) {
    self.transition(State::reject);
}

fn transition(&mut self, f: fn(Box<dyn State>) -> Box<dyn State>) {
    self.state = self.state.take().map(f);
}
```
