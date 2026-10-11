A type that implements `Drop` cannot be taken apart: `drop` will still run on the value later, and
it gets `&mut self` with every field in place. So both `let name = self.name;` and the functional
record update `TempFile { name: ..., ..self }` are rejected (E0509): `..self` moves `data` and
`log` out of `self` just like a field access does. Taking `self` by value does not switch `Drop`
off. Inside the method, `self` is an ordinary local variable that is dropped at the end unless it
is moved somewhere as a whole.

`rename` is easy once you see that: change the name in place and return `self` as a whole, which is
allowed (`mut` on a parameter is not part of the signature). The compiler suggests cloning the
fields instead, which compiles but builds a second file, and the old one is dropped at the end of
`rename`, which logs `"delete draft (3 bytes)"`.

The real trap is `mem::forget(self)` in `persist`. It skips `drop`, but it also skips the
destructors of **all the fields**. Taking `name` and `data` out with `std::mem::take` and then
forgetting `self` looks clean, but `log` is still inside: its `Rc` is never released,
`Rc::strong_count(&log)` never goes back to 1, and Miri reports the leak. Keeping `forget` and
patching around it in safe code fails too: `let log = self.log.clone()` before `forget` adds one
count and later removes only that one, and `drop(mem::take(&mut self.log))` releases the real log
but puts a brand-new `Rc` (from `Rc::default()`) in its place, which `forget` then leaks.
Forgetting a value is only harmless when none of its remaining fields own anything.

So when `persist` skips `TempFile::drop`, it still has to give **every** field a new owner, the log
included. (If the fields and the `Drop` impl could change, a flag that `drop` checks would do, but
the statement rules that out.) The solution wraps `self` in `ManuallyDrop`, which never drops its
contents, and moves the fields out one by one with `ptr::read`. Each value then has exactly one
owner: `name` and `data` are returned, and `log` is dropped normally, which releases the handle.
The fields have to be read individually: a copy of the whole struct made with `ptr::read` cannot be
destructured either (E0509 again), and `ManuallyDrop::into_inner` or `ManuallyDrop::drop` would run
`TempFile::drop` after all. Wrapping `self` first, before anything is read, also guarantees that
nothing can be dropped twice if a panic happened in between.

### Alternatives

The template's `forget` can stay if `log` is read out of `self` first. Here nothing can panic
between `ptr::read` and `forget`; if something could, the unwinding would drop both `log` and
`self`, releasing the same handle twice. `ManuallyDrop` rules that out by construction.

```rust
pub fn persist(mut self) -> (String, Vec<u8>) {
    let name = std::mem::take(&mut self.name);
    let data = std::mem::take(&mut self.data);
    // SAFETY: `self` is forgotten right after this, so `log` becomes the only owner of
    // this file's handle to the log.
    let log = unsafe { std::ptr::read(&self.log) };
    std::mem::forget(self);
    drop(log);
    (name, data)
}
```

`drop` always runs unless you skip it, so another way is to make sure it cannot do any harm, without
`unsafe`. `persist` replaces the log with a new, empty one: the real log is released right away,
and `drop` writes its `delete` line into the throwaway log, which is freed together with `self`. It
passes, but it hides the intent and allocates a log only to discard it.

```rust
pub fn persist(mut self) -> (String, Vec<u8>) {
    let name = std::mem::take(&mut self.name);
    let data = std::mem::take(&mut self.data);
    self.log = Log::default();
    (name, data)
}
```
