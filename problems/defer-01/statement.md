A *deferred action* is code that must run when the current scope is left, however it is left: at
its end, by an early `return`, or by a panic. Go has a `defer` statement for this. In Rust, it can
be a guard value that runs the action when it is dropped. Implement such a guard, `Defer`, and
`defer_all`, which bundles several actions into one guard.

```rust
pub struct Defer<F> { /* fields up to you */ }

pub fn defer<F: FnOnce()>(f: F) -> Defer<F>;

impl<F: FnOnce()> Defer<F> {
    pub fn cancel(self);
    pub fn run_now(self);
}

impl<F> Drop for Defer<F> { /* to implement */ }

pub fn defer_all(actions: Vec<Box<dyn FnOnce()>>) -> Defer<impl FnOnce()>;
```

- `defer(f)` returns a guard that holds the action `f`. It does not call `f`.
- A guard runs its action when it is dropped: at the end of its scope, by `drop(guard)`, or while a
  panic unwinds the stack. Like any local variables, several guards in one scope are dropped in the
  reverse order of their declaration, so their actions run from the last guard to the first. Moving
  a guard does not run its action: it runs when the guard's last owner drops it.
- `cancel` consumes the guard without running its action. The action is dropped right away, so
  everything it captured is released before `cancel` returns.
- `run_now` consumes the guard and runs its action immediately. Nothing runs again later.
- An action runs at most once. If it panics, the panic propagates to the code that dropped the guard
  (or called `run_now`), and the action is not run again.
- An action can be any value whose type implements `FnOnce()`: a closure that borrows local
  variables, a `move` closure, a closure that can be called only once (for example because it gives
  away a captured `String`), or a `Box<dyn FnOnce()>`.
- `defer_all(actions)` returns one guard whose action runs all of `actions`, from the **last**
  element to the first (`actions` may be empty). None of them runs before that. `cancel` on this
  guard runs none of them and releases them all right away, and `run_now` runs them all immediately.
- If one of these actions panics, the remaining ones still run, in the same order (last to first).
  Then the panic continues, with the same payload, out of the `drop` (or `run_now`) of the guard.
- The tests never make an action panic while another panic is already unwinding (Rust would abort
  the program), and at most one of the actions given to `defer_all` panics.

The code above is already in the editor. Keep the signatures of `defer`, `cancel`, `run_now` and
`defer_all`. The definition of `Defer<F>` and its `Drop` impl are up to you.

### Examples

```rust
let log = RefCell::new(Vec::new());
{
    let _a = defer(|| log.borrow_mut().push("a"));
    let _b = defer(|| log.borrow_mut().push("b"));
    log.borrow_mut().push("body");
} // _b is dropped first, then _a
// log: ["body", "b", "a"]

defer(|| log.borrow_mut().push("never")).cancel(); // appends nothing
defer(|| log.borrow_mut().push("now")).run_now();  // appends "now" right away
```

```rust
let log = RefCell::new(Vec::new());
let result = catch_unwind(AssertUnwindSafe(|| {
    let _g = defer(|| log.borrow_mut().push("cleanup"));
    panic!("oops");
}));
// result is an Err with the payload "oops", and log is ["cleanup"]
```

In the next examples, `log` is an `Rc<RefCell<Vec<&str>>>`, and `push(&log, s)` returns a
`Box<dyn FnOnce()>` that appends `s` to `log`.

```rust
let all = defer_all(vec![push(&log, "1"), push(&log, "2"), push(&log, "3")]);
// log is still empty
drop(all);
// log: ["3", "2", "1"]
```

```rust
let result = catch_unwind(AssertUnwindSafe(|| {
    let _all = defer_all(vec![push(&log, "1"), Box::new(|| panic!("2")), push(&log, "3")]);
}));
// log: ["3", "1"], and result is an Err with the payload "2"
```
