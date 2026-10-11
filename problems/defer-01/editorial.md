Calling an `FnOnce` consumes it, but `drop` only gets `&mut self`, and a field cannot be moved out
from behind a reference (E0507). So the guard keeps its action in an `Option<F>`, and `drop` calls
`self.action.take()`, which moves the action out and leaves `None` behind. `None` also means "nothing
left to do", which keeps `cancel` and `run_now` simple.

To call the action in `drop`, the `Drop` impl needs `F: FnOnce()`. But a `Drop` impl may not require
more than the struct itself: it must work for every `Defer<F>` that can exist, and Rust has no "drop
only when `F: FnOnce()`". `impl<F: FnOnce()> Drop for Defer<F>` on a struct declared as `Defer<F>` is
rejected (E0367). The solution puts the same bound on the struct: `pub struct Defer<F: FnOnce()>`.
Bounds usually belong on impls rather than on struct definitions, but a `Drop` impl that needs one is
the exception.

`cancel` and `run_now` take `self` by value, but `Defer` implements `Drop`, so its field cannot be
moved out: `let Defer { action } = self;` and `self.action.unwrap()` are rejected (E0509), because
`drop` still runs on `self` at the end of the method. With `mut self` (the `mut` is not part of the
signature), `cancel` sets `self.action = None`. That drops the closure, and everything it captured,
right there, and `drop` later finds nothing to run. `std::mem::forget(self)` compiles as well, but
it never drops the closure: an `Rc` that the closure captured stays alive forever, and Miri reports
the leak. `run_now` is simply `drop(self)`, since running the action is what dropping does.

In `defer_all`, the closure passed to `defer` takes ownership of `actions`. A closure type has no
name, and the return type `Defer<impl FnOnce()>` lets the function return a guard around one anyway.
The hard part is the panic rule. A plain loop `for action in actions.into_iter().rev() { action() }`
stops at the first panic: the iterator is dropped while the panic unwinds, and dropping the remaining
boxes does not call them. The solution turns each action into a guard and lets drop glue do the
work: when the destructor of one element panics, Rust still drops the remaining elements of the
`Vec` (just as it drops the remaining fields of a struct, or the remaining locals of a scope) while
the panic unwinds. Two details matter:

- The guards are created inside the closure, when it runs. If `defer_all` created them and the
  closure captured them, `cancel` would drop the closure, and with it the guards, which would run
  every action.
- The order comes from `rev()` before the guards are created, because a `Vec` drops its elements
  from front to back. Dropping them one by one in `guards.into_iter().rev()` is wrong after a panic:
  the `IntoIter` drops the elements it has not yielded yet from front to back, so `[1, 2, 3, 4, 5]`
  with a panic in `4` runs `5, 1, 2, 3`.

When you use a guard, bind it to a named variable such as `_g`. `let _ = defer(...)` binds nothing,
so the guard is dropped, and its action runs, immediately.

### Alternatives

`Defer<F>` can stay free of bounds if it stores a function pointer that calls the action. `defer`
creates it, because `F: FnOnce()` is known there. A closure that captures nothing coerces to a `fn`
pointer. `cancel`, `run_now` and `defer_all` stay as in the solution.

```rust
pub struct Defer<F> {
    action: Option<F>,
    /// Calls the action. Set by `defer`, where `F: FnOnce()` is known.
    call: fn(F),
}

pub fn defer<F: FnOnce()>(f: F) -> Defer<F> {
    Defer { action: Some(f), call: |f| f() }
}

impl<F> Drop for Defer<F> {
    fn drop(&mut self) {
        if let Some(action) = self.action.take() {
            (self.call)(action);
        }
    }
}
```

`defer_all` can catch each panic with `catch_unwind` and continue the first one with `resume_unwind`
once every action has run. `AssertUnwindSafe` is needed because a `Box<dyn FnOnce()>` is not
`UnwindSafe`. Unlike the solution, this version also copes with several panicking actions, where the
drop glue would abort on the second panic.

```rust
use std::panic::{self, AssertUnwindSafe};

pub fn defer_all(actions: Vec<Box<dyn FnOnce()>>) -> Defer<impl FnOnce()> {
    defer(move || {
        let mut first_panic = None;
        for action in actions.into_iter().rev() {
            if let Err(payload) = panic::catch_unwind(AssertUnwindSafe(action)) {
                first_panic.get_or_insert(payload);
            }
        }
        if let Some(payload) = first_panic {
            panic::resume_unwind(payload);
        }
    })
}
```

Or let a guard run the rest: take the last action, defer running the others, then call it. If it
panics, the guard runs the others while the panic unwinds.

```rust
/// Runs `actions` from the last one to the first.
fn run_from_last(mut actions: Vec<Box<dyn FnOnce()>>) {
    if let Some(last) = actions.pop() {
        // Runs the others after `last`, even if `last` panics.
        let _rest = defer(move || run_from_last(actions));
        last();
    }
}

pub fn defer_all(actions: Vec<Box<dyn FnOnce()>>) -> Defer<impl FnOnce()> {
    defer(move || run_from_last(actions))
}
```
