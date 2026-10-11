A type that implements `Drop` cannot be taken apart: `drop` will still run on the value later, and
it gets `&mut self` with every field in place. So both `let name = self.name;` and the functional
record update `TempFile { name: ..., ..self }` are rejected (E0509): `..self` moves `data` and
`log` out of `self` just like a field access does. Taking `self` by value does not switch `Drop`
off. Inside the method, `self` is an ordinary local variable that is dropped at the end unless it
is moved somewhere as a whole.

The compiler suggests cloning, which compiles but is wrong. Cloning the fields in `rename` builds a
second file, and the old one is dropped at the end of `rename`, which logs
`"delete draft (3 bytes)"`. Cloning in `persist` keeps the originals inside the forgotten value,
and they leak.

The real trap is `mem::forget(self)`. It skips `drop`, but it also skips the destructors of **all
the fields**. Taking `name` and `data` out with `std::mem::take` (which needs `mut self`) and then
forgetting `self` looks clean, but `log` is still inside: its `Rc` is never released,
`Rc::strong_count(&log)` never goes back to 1, and Miri reports the leak. Patching it fails too:
`let log = self.log.clone()` before `forget` adds one count and later removes only that one, and
`drop(mem::take(&mut self.log))` releases the real log but puts a brand-new `Rc` (from
`Rc::default()`) in its place, which `forget` then leaks. Forgetting a value is only harmless when
none of its remaining fields own anything.

The solution does not skip `drop` at all. It adds a `persisted` flag: `persist` sets it and takes
out the name and the contents, `self` is dropped normally at the end of the method (which releases
`log`), and `drop` logs nothing for a persisted file. This is a hand-made drop flag: the compiler
tracks by itself whether a *variable* has been moved out of (and drops it only if not), but only
the type knows whether its `drop` still has work to do. `rename` needs no flag at all. It changes
the name in place and returns `self` as a whole, which is allowed; only moving fields out is not.
`mut` on a parameter is not part of the signature.

### Alternatives

To skip `drop` as the template wanted, wrap the file in `ManuallyDrop` and move **every** field
out with `ptr::read`, `log` included. Reading only `name` and `data` leaks the log exactly as
`forget` did, and `ManuallyDrop::drop` or `ManuallyDrop::into_inner` would run `TempFile::drop`
after all. `rename` stays as in the solution, and `TempFile` keeps its three fields.

```rust
use std::mem::ManuallyDrop;
use std::ptr;

// in `impl TempFile`
pub fn persist(self) -> (String, Vec<u8>) {
    // Neither `drop` nor the destructors of the fields run on `file`.
    let file = ManuallyDrop::new(self);
    // SAFETY: each field is read exactly once and `file` is never used again,
    // so nothing is dropped twice.
    let (name, data, log) =
        unsafe { (ptr::read(&file.name), ptr::read(&file.data), ptr::read(&file.log)) };
    drop(log);
    (name, data)
}
```

`drop` always runs unless you skip it, so another way is to make sure it cannot do any harm. With
the original three fields (and `rename` from the solution), `persist` can replace the log with a
new, empty one: the real log is released right away, and `drop` writes its `delete` line into the
throwaway log, which is freed together with `self`. It passes, but it hides the intent and
allocates a log only to discard it.

```rust
pub fn persist(mut self) -> (String, Vec<u8>) {
    let name = std::mem::take(&mut self.name);
    let data = std::mem::take(&mut self.data);
    self.log = Log::default();
    (name, data)
}
```
