`set_len(n)` promises that the first `n` elements are initialized, but the buffer that
`with_capacity` returns is not. From then on, everything that touches those slots treats garbage as
a `T`:

- `v[i] = f(i)` is an assignment, and an assignment **drops the old value** before it stores the new
  one. For an `i32` that does nothing, which is why the template works for numbers. For a `String`,
  it reads a garbage pointer and capacity and tries to free them, and Miri reports a read of
  uninitialized memory.
- Writing with `ptr::write` (`v.as_mut_ptr().add(i).write(f(i))`) does not drop anything, but that
  alone is not enough. If `f` panics, unwinding drops `v`, and with its length already at `n`, `v`
  also drops the slots that were never written.
- Setting the length only after the loop avoids that, but then a panic leaves `v` with length 0: it
  drops nothing, and the values written so far are leaked.
- When a vector is filled from the front, the usual fix is to grow the length after each write,
  which is what `push` does. Here the slots are filled out of order, so the values written so far
  are not necessarily in the first slots.

When `f` panics, the values produced so far sit in scattered slots, and something must know which
ones. The model solution leaves that to the type system: it starts with a `Vec<Option<T>>` of
`None`s and stores `Some(f(i))`. `slots` is an ordinary local variable, so if `f` panics, unwinding
drops it, and it drops exactly the values in it. Assigning to `slots[i]` is fine too, because it
only drops a `None`. At the end every slot is `Some`, and `into_iter().map(...).collect()` moves the
values into a `Vec<T>`. No `unsafe` is needed, at the cost of a second allocation.

Watch out for `vec![None; n]`: it clones its element, so it requires `Option<T>: Clone`, that is,
`T: Clone`, which `build` does not have. `order.iter().map(|_| None).collect()` or
`iter::repeat_with(|| None).take(n).collect()` creates the `None`s without cloning.

### Alternatives

Collect `(index, value)` pairs in the order of the calls, then sort them by index. This is
panic-safe by itself, too: if `f` panics, the vector that `collect` was building is dropped with the
pairs in it.

```rust
pub fn build<T>(order: &[usize], mut f: impl FnMut(usize) -> T) -> Vec<T> {
    let mut pairs: Vec<(usize, T)> = order.iter().map(|&i| (i, f(i))).collect();
    pairs.sort_unstable_by_key(|&(i, _)| i);
    pairs.into_iter().map(|(_, value)| value).collect()
}
```

With `unsafe`, write the values into the spare capacity of a `Vec` whose length stays 0, and keep
a **guard** that knows which slots hold values: `order[..written]`. If `f` panics, the guard is
dropped while unwinding and drops exactly those values, and then `v` frees its buffer. Once every
slot is written, `mem::forget` disarms the guard, and `set_len(n)` hands the values over to `v`.
Note the type annotation on `v`: without one, `v.as_mut_ptr().add(i)` does not compile, because the
type of the elements is not known yet at that point.

```rust
use std::mem;

/// Owns the values written so far into the buffer of `v`, whose length is still 0: they are in
/// the slots `order[..written]`. Dropping the guard drops exactly those values.
struct Written<'a, T> {
    v: &'a mut Vec<T>,
    order: &'a [usize],
    written: usize,
}

impl<T> Drop for Written<'_, T> {
    fn drop(&mut self) {
        let p = self.v.as_mut_ptr();
        for &i in &self.order[..self.written] {
            unsafe { p.add(i).drop_in_place() };
        }
    }
}

pub fn build<T>(order: &[usize], mut f: impl FnMut(usize) -> T) -> Vec<T> {
    let n = order.len();
    let mut v: Vec<T> = Vec::with_capacity(n);
    let mut guard = Written { v: &mut v, order, written: 0 };
    for &i in order {
        let value = f(i);
        // `write` does not drop the old (uninitialized) contents of the slot.
        unsafe { guard.v.as_mut_ptr().add(i).write(value) };
        guard.written += 1;
    }
    // Every slot holds a value now: disarm the guard and hand the values over to `v`.
    mem::forget(guard);
    unsafe { v.set_len(n) };
    v
}
```

`written` must count only values that are really in place: incrementing it before `f(i)` returns
would make the guard drop an empty slot. This version is also sound only because `order` is a
permutation. An index out of range would write past the buffer, and a repeated index would leave a
slot uninitialized. A real public function would have to check `order` first.
