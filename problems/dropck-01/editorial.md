There are two errors, and fixing only one of them is not enough.

First, `sink.into_inner()` moves `sink` while it is still borrowed (E0505). A borrow usually ends at
its last use, but `Tracer` implements `Drop`, so the compiler assumes that dropping a tracer may use
`self.sink`. The tracers therefore keep `sink` borrowed until they are dropped, and locals are
dropped at the closing brace, **after** the tail expression `sink.into_inner()` has been evaluated.
Even if this compiled, the `"end"` lines would be written too late to be part of the result. Drop
the tracers explicitly before moving `sink` out: `drop(tracers)`. A `Vec` drops its elements front
to back, which is the required order.

Second, `drop(tracers)` alone still does not compile: "`sink` does not live long enough" (E0597).
Locals are dropped in the reverse order of their declaration, so `sink`, declared after `tracers`,
would be dropped first. On the normal path `tracers` has already been moved into `drop` by then, but
the borrow checker also checks the paths taken when a panic unwinds the stack. If `Tracer::new`,
`push` or `borrow_mut` panicked before `drop(tracers)`, unwinding would drop `sink` and then
`tracers`, whose `Drop` would write into the destroyed sink. This is the **drop check**: when a
value's type implements `Drop`, everything it borrows must strictly outlive it. Declaring `sink`
before `tracers` fixes it.

All of this comes from the `Drop` impl. Without it, the compiler knows that dropping a `Tracer`
never touches `sink`, and the template would compile (with no `"end"` lines, of course).

Two tempting fixes do not work:

- `tracers.clear()` or `std::mem::take(&mut tracers)` writes the `"end"` lines, but the (now empty)
  `Vec<Tracer<'_>>` lives until the end of the function, and dropping it still counts as a use of
  the borrow. The vector itself must be moved away or go out of scope.
- Replacing `sink.into_inner()` with `sink.take()` or `sink.borrow().clone()` compiles once `sink`
  comes first, but the tracers are then dropped after the lines were taken, and the `"end"` lines
  are lost.

### Alternatives

Put the tracers in an inner block. They are dropped at its closing brace, before `sink` is moved.

```rust
pub fn trace(names: &[&str]) -> Vec<String> {
    let sink = RefCell::new(Vec::new());
    {
        let mut tracers = Vec::new();
        for name in names {
            tracers.push(Tracer::new(&sink, name));
        }
        sink.borrow_mut().push("run".to_string());
    }
    sink.into_inner()
}
```

`RefCell::take` also works, as long as the tracers are dropped first.

```rust
pub fn trace(names: &[&str]) -> Vec<String> {
    let sink = RefCell::new(Vec::new());
    let tracers: Vec<_> = names.iter().map(|name| Tracer::new(&sink, name)).collect();
    sink.borrow_mut().push("run".to_string());
    drop(tracers);
    sink.take()
}
```
