`less` is the caller's code, and it may panic. The panic then unwinds out of `insert_sorted` right
at the call to `less`, leaving the vector in whatever state it is in at that moment, and the vector
is dropped later as usual. In the template, that state is broken during the whole loop:

- `ptr::read` makes a bitwise copy, so `x` is owned twice: by `tmp` and by the last slot.
- After each `ptr::copy_nonoverlapping`, one element sits in two slots.

If `less` panics on its first call, unwinding drops `tmp`, and dropping `v` later drops `x` again.
If it panics after a shift, dropping `v` drops the shifted element twice. For a `String`, this is a
double free, and Miri reports a use-after-free when `v` is dropped.

This is called *exception safety* (see "Exception Safety" in the Rustonomicon): whenever code you do
not control runs (a closure, or a trait method such as `Ord::cmp`, `Clone::clone` or `Drop::drop`),
everything must be in a state that is safe to drop. Wrapping `tmp` in `ManuallyDrop` is not enough:
it fixes a panic on the first call, but after a shift, one element is still in two slots.

The model solution works in two phases. First it only looks: `partition_point` does a binary search
and does not change `v`. If `less` panics there, `v` is intact, and `x`, which is still a local
variable of the function, is dropped once while unwinding. Then it changes `v` with `Vec::insert`,
which only moves values and never calls the caller's code.

The predicate `!less(&x, e)` is true for the elements that are not greater than `x`. In a sorted
vector they come first, so `partition_point` returns the position after the last element equal to
`x`. With `|e| less(e, &x)` you would get the position *before* the equal elements instead. Do not
rely on `binary_search_by` here: when several elements are equal, its documentation allows it to
return any one of them.

Catching the panic with `std::panic::catch_unwind` does not help: the panic has to reach the caller.

### Alternatives

Push `x` and move it forward with swaps, like insertion sort does. Each `swap` is a complete
operation, so the vector is valid whenever `less` runs, and after a panic, `x` simply stays where it
is. No `unsafe` is needed.

```rust
pub fn insert_sorted<T, F: FnMut(&T, &T) -> bool>(v: &mut Vec<T>, x: T, mut less: F) {
    v.push(x);
    let mut i = v.len() - 1;
    while i > 0 && less(&v[i], &v[i - 1]) {
        v.swap(i, i - 1);
        i -= 1;
    }
}
```

A linear search works in the two-phase solution too, with more calls to `less`:
`let pos = v.iter().position(|e| less(&x, e)).unwrap_or(v.len());`.

You can also keep the template's shifting and make it panic-safe with a guard that owns the gap,
like the `Hole` that the standard library uses in `BinaryHeap`. The guard's `Drop` writes the value
into the gap. It also runs while unwinding, so the vector is valid however the function is left.

```rust
use std::mem::ManuallyDrop;
use std::ptr;

/// While a `Hole` is alive, the slot `pos` of `v` is logically empty, and the value that belongs
/// there is kept in `value`. Dropping the hole, normally or while unwinding, moves the value back
/// into the slot, so `v` is valid again.
struct Hole<'a, T> {
    v: &'a mut [T],
    value: ManuallyDrop<T>,
    pos: usize,
}

impl<T> Drop for Hole<'_, T> {
    fn drop(&mut self) {
        unsafe {
            ptr::copy_nonoverlapping(&*self.value, self.v.as_mut_ptr().add(self.pos), 1);
        }
    }
}

pub fn insert_sorted<T, F: FnMut(&T, &T) -> bool>(v: &mut Vec<T>, x: T, mut less: F) {
    v.push(x);
    let last = v.len() - 1;
    let value = ManuallyDrop::new(unsafe { ptr::read(&v[last]) });
    let mut hole = Hole { v: v.as_mut_slice(), value, pos: last };
    while hole.pos > 0 && less(&hole.value, &hole.v[hole.pos - 1]) {
        let p = hole.v.as_mut_ptr();
        unsafe { ptr::copy_nonoverlapping(p.add(hole.pos - 1), p.add(hole.pos), 1) };
        hole.pos -= 1;
    }
    // `hole` is dropped here and writes the value into its final slot.
}
```
