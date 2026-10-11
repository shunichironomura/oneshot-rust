`self.cache.entry(id)` mutably borrows `self.cache` until the returned reference is no longer used.
The closure passed to `or_insert_with` calls `self.build(id)`, a `&mut self` method, so it needs
unique access to **all** of `self`, including `self.cache`, which is already borrowed. The compiler
cannot see that `build` only touches `self.built`: a method signature borrows the whole struct.

Do the cache lookup and the build one after the other instead of nesting them. Check
`contains_key`, build and insert the label only when it is missing, and then return a reference into
the cache.

Moving the call out of the closure (`let label = self.build(id);` before `entry`) compiles too, but
then a label is built on every call and `built` is wrong.

### Alternatives

Keep the entry API, but let the closure borrow only the field it needs. Inside a method body the
compiler tracks the fields separately, and closures capture only the fields they use, so writing
`self.built += 1` directly in the closure is fine.

```rust
pub fn get(&mut self, id: u32) -> &str {
    self.cache.entry(id).or_insert_with(|| {
        self.built += 1;
        format!("item-{id}")
    })
}
```

This does a single hash lookup instead of two. If you want to keep a separate helper, make it take
`&mut usize` (or whatever fields it needs) instead of `&mut self`.
