Two parts of `Ring` are harder than they look: creating the array, and moving elements into and out
of it.

`new` cannot use `[None; N]`. A repeat expression `[x; N]` copies `x` into every element, so `x`
must be `Copy`, and `Option<T>` is `Copy` only when `T` is. Generic code is type-checked once for
all possible `T`, so it does not matter that the example uses `i32`: the compiler rejects
`[None; N]` with E0277. Adding the bound `T: Copy`, as the compiler suggests, only moves the error
to the harness, which uses `String`. A constant is the exception: an inline `const` block (stable
since Rust 1.79) is evaluated again for each element instead of being copied, so
`[const { None }; N]` works for any `T`. `Default::default()` does not work either, because arrays
implement `Default` only for lengths up to 32, not for every `N`.

`self.buf[i]` names a place inside the array. `let x = self.buf[i];` would move the element out and
leave a hole, so it is rejected (E0507). `Option` can swap values instead: `take()` leaves `None`
behind and returns the old value, and `replace(x)` puts `Some(x)` in and returns the old value.
`pop` takes the oldest element out, and `push` on a full ring replaces the oldest element with the
new one in the same slot.

The rest is index arithmetic modulo `N`. The oldest element is at `start`, the element `i` places
after it at `(start + i) % N`, and the first free slot at `(start + len) % N`. After a pop or an
eviction, `start` moves one slot forward. `iter` maps the indices `0..len` to references into `buf`,
so the iterator borrows the ring. That is what `+ '_` says (in edition 2024, an `impl Trait` return
type captures that lifetime even without it).

### Alternatives

`std::array::from_fn` calls a closure for every index, so it does not need `Copy` either.

```rust
pub fn new() -> Self {
    Ring { buf: std::array::from_fn(|_| None), start: 0, len: 0 }
}
```

Before inline `const` blocks, the usual trick was a named constant: a path to a constant item is
also allowed as the operand of a repeat expression.

```rust
impl<T, const N: usize> Ring<T, N> {
    pub const CAPACITY: usize = N;
    const EMPTY: Option<T> = None;

    pub fn new() -> Self {
        Ring { buf: [Self::EMPTY; N], start: 0, len: 0 }
    }

    // The other methods stay the same.
}
```

`push` can reuse `pop` to make room.

```rust
pub fn push(&mut self, x: T) -> Option<T> {
    let evicted = if self.is_full() { self.pop() } else { None };
    self.buf[(self.start + self.len) % N] = Some(x);
    self.len += 1;
    evicted
}
```

Since every free slot holds `None`, `iter` can also walk the array from `start` to the end and then
from the beginning, and skip the `None`s with `flatten` (a `&Option<T>` iterates over zero or one
`&T`).

```rust
pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
    let (wrapped, oldest_first) = self.buf.split_at(self.start);
    oldest_first.iter().chain(wrapped).flatten()
}
```
