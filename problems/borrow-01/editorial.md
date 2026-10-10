`v.iter()` borrows `v` immutably for the whole loop, so `v.push(...)` (a mutable borrow) cannot happen
inside it. The fix is to **finish reading before you start writing**: collect the new values into a
separate `Vec` first, then append them with `extend`.

### Alternatives

Fix the number of elements before the loop and access them by index. Each `v[i]` is a short borrow
that ends immediately, so pushing in the loop body is fine.

```rust
pub fn push_doubled(v: &mut Vec<i32>) {
    for i in 0..v.len() {
        if v[i] > 0 {
            v.push(v[i] * 2);
        }
    }
}
```

`0..v.len()` is evaluated only once, so the loop never visits the elements it appended.
