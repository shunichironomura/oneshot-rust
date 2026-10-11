The template builds `whole`, a second `&mut [i32]` covering **the entire slice**, and then keeps
using `v` as well. A `&mut` promises exclusive access, so two of them over the same memory cannot
both stay usable, even though the halves the caller finally sees do not overlap. Here
`&mut v[..mid]` reborrows all of `v` uniquely, which invalidates `whole`, and Miri's Stacked Borrows
check reports the use of `whole` on the next line.

The fix is to never create a reference that covers memory someone else is using. Take one raw
pointer and build each half **from that pointer**, with exactly the range it owns:
`from_raw_parts_mut(ptr, mid)` and `from_raw_parts_mut(ptr.add(mid), len - mid)`. The two ranges
are disjoint, so the two `&mut` never alias.

### Alternatives

The standard library already provides this operation safely, so the whole function can be a single
call.

```rust
pub fn split_in_two(v: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    v.split_at_mut(mid)
}
```

Its implementation is essentially the model solution: one pointer, two non-overlapping ranges.
