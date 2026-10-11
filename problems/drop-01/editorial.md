Local variables are dropped in the **reverse** order of their declaration, but the elements of a
`Vec` are dropped from the **first** to the last (so are the fields of a struct and the elements of
an array or a tuple). `drop(locks)`, or simply letting `locks` go out of scope, therefore releases
`a`, `b`, `c` in that order, which is wrong for `release_all`. Pop the locks from the end instead.

`with_lock` hides the other trap. `let _ = Lock::acquire(name, log);` looks as if it kept the lock
until the end of the function, but `_` is not a variable: the pattern matches the value without
binding it, so nothing owns the new lock, and it is dropped at the end of the statement, before
`f` runs. The expression statement `Lock::acquire(name, log);` does the same. A named variable owns
the lock until the end of its scope, and a leading underscore does not change that: `_lock` is a
real variable, and the underscore only silences the "unused variable" warning. The tail expression
`f()` is evaluated before the local variables are dropped, so its result can be returned directly,
and the lock is released right after `f` returns.

`swap_in` needs no special care. In `*slot = Lock::acquire(name, &slot.log)` the right-hand side
is evaluated first, and then the assignment drops the old value of `*slot`. The shared borrow of
`slot.log` ends when `acquire` returns, because its return type `Lock` borrows nothing (the new lock
holds its own clone of the `Rc`), so the assignment to `*slot` is allowed.

`keep_first` uses the same front-to-back rule as `release_all`, and this time it is the order we
want. `into_iter()` moves the locks into an iterator and `next()` takes out the first one. The
iterator still owns the others, and when it is dropped at the end of the function, it drops them in
order. Be careful with `swap_remove(0)`, which moves the last lock to the front.

### Alternatives

`with_lock` can release the lock explicitly, which makes the order visible.

```rust
pub fn with_lock<R>(name: &str, log: &Log, f: impl FnOnce() -> R) -> R {
    let lock = Lock::acquire(name, log);
    let result = f();
    drop(lock);
    result
}
```

`release_all` can also drop the locks through a reversed iterator.

```rust
pub fn release_all(locks: Vec<Lock>) {
    locks.into_iter().rev().for_each(drop);
}
```

`std::mem::replace` spells out what the assignment in `swap_in` does.

```rust
pub fn swap_in(slot: &mut Lock, name: &str) {
    let new = Lock::acquire(name, &slot.log);
    let old = std::mem::replace(slot, new);
    drop(old);
}
```
