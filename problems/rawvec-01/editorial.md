Everything in the buffer past `len` is **uninitialized memory**. Most of the traps come from
treating it as if it already held a `T`.

- `push` must store the value with `ptr::write` (here `NonNull::write`). An assignment `*slot = x`
  first drops the old value in the slot: for a `String`, that reads garbage as a pointer and frees
  it, and Miri reports a read of uninitialized memory. `pop` is the mirror image: decrement `len`,
  then move the value out with `ptr::read`. The slot is no longer counted, so it is never dropped
  twice.
- `Layout::array::<T>(cap)` gives the size and alignment of the buffer. The first buffer comes from
  `alloc`. After that, `realloc(ptr, old_layout, new_size)` takes the layout the block **was
  allocated with**, not the new one, and `dealloc` needs that exact layout too. A null result means
  the allocation failed, and `handle_alloc_error` reports it.
- An empty `MyVec` holds `NonNull::dangling()`. It is non-null and well aligned, which is all a
  slice of length 0 needs, but it points to no allocation: `Drop` must not `dealloc` it when
  `cap == 0`.
- `Drop` drops the initialized part as one slice with `ptr::drop_in_place`, then frees the buffer.

The given `get` needs no `unsafe` of its own: `as_slice` builds the slice once with
`slice::from_raw_parts`, and the slice's `get` does the bounds check.

### Alternatives

Instead of `realloc`, `grow` can allocate a new buffer, move the elements over with
`ptr::copy_nonoverlapping` and free the old buffer (copying bytes moves the values; freeing the old
block drops nothing). `Drop` can pop until the `MyVec` is empty, which moves each element out and
drops it.

```rust
fn grow(&mut self) {
    let new_cap = if self.cap == 0 { 4 } else { 2 * self.cap };
    let new_layout = Layout::array::<T>(new_cap).expect("capacity overflow");
    // SAFETY: T is not zero-sized and new_cap > 0.
    let new_ptr = unsafe { alloc::alloc(new_layout) }.cast::<T>();
    let Some(new_ptr) = NonNull::new(new_ptr) else {
        alloc::handle_alloc_error(new_layout);
    };
    if self.cap > 0 {
        // SAFETY: the old buffer holds `len` initialized values and the new one is larger.
        // After the copy they belong to the new buffer; the old one is freed with its own
        // layout, which does not drop anything.
        unsafe {
            ptr::copy_nonoverlapping(self.ptr.as_ptr(), new_ptr.as_ptr(), self.len);
            alloc::dealloc(self.ptr.as_ptr().cast(), Layout::array::<T>(self.cap).unwrap());
        }
    }
    self.ptr = new_ptr;
    self.cap = new_cap;
}
```

```rust
impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
        if self.cap > 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            // SAFETY: `ptr` was allocated with exactly this layout.
            unsafe { alloc::dealloc(self.ptr.as_ptr().cast(), layout) };
        }
    }
}
```
