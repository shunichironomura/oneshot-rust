Local variables are dropped in the **reverse** order of their declaration, but the elements of a
`Vec` are dropped from the **first** to the last (so are the fields of a struct and the elements of
an array or a tuple). `drop(locks)`, or simply letting `locks` go out of scope, therefore releases
`a`, `b`, `c` in that order, which is wrong for `release_all`. Pop the locks from the end instead.

`swap_in` needs no special care. In `*slot = Lock::acquire(name, &slot.log)` the right-hand side
is evaluated first, and then the assignment drops the old value of `*slot`. The borrow of
`slot.log` ends when `acquire` returns, because the new lock holds its own clone of the `Rc`.

`keep_first` uses the same front-to-back rule, and this time it is the order we want.
`into_iter()` moves the locks into an iterator and `next()` takes out the first one. The iterator
still owns the others, and when it is dropped at the end of the function, it drops them in order.
Be careful with `swap_remove(0)`, which moves the last lock to the front.

The trap is `into_name`. Returning `self.name` does not compile: you cannot move a field out of a
type that implements `Drop` (E0509), because `drop` still runs on the value later and expects every
field to be there. The compiler suggests `self.name.clone()`, which compiles but releases the lock
when `self` is dropped. Instead, take the name out and leave something behind that `drop`
recognizes. The solution makes the field an `Option<String>`: `take()` leaves `None`, and `drop`
logs nothing for `None`.

`std::mem::forget(self)` also compiles and skips `drop`, but it skips dropping the fields as well.
The `Rc` in `log` is never released, `Rc::strong_count(&log)` stays above 1, and Miri reports a
memory leak. Taking the name with `std::mem::take` and making `drop` skip empty names is not enough
either, because the empty string is a valid name.

### Alternatives

Keep `name: String` and add a flag. `std::mem::take` leaves an empty `String` behind, and the flag
tells `drop` that the lock was consumed.

```rust
pub struct Lock {
    name: String,
    log: Log,
    armed: bool,
}

impl Lock {
    pub fn acquire(name: &str, log: &Log) -> Lock {
        log.borrow_mut().push(format!("acquire {name}"));
        Lock { name: name.to_string(), log: Rc::clone(log), armed: true }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn into_name(mut self) -> String {
        self.armed = false;
        std::mem::take(&mut self.name)
    }
}

impl Drop for Lock {
    fn drop(&mut self) {
        if self.armed {
            self.log.borrow_mut().push(format!("release {}", self.name));
        }
    }
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
