`self.items()` returns a slice that borrows **all of `self`**, not just `self.items`: the signature
`fn items(&self) -> &[Item]` is all the caller sees. The loop holds that borrow until it ends, so
calling `self.record(...)`, which takes `&mut self`, inside the loop is rejected. Writing
`self.total += ...` directly in the loop would be rejected for the same reason.

Inside one function, though, the compiler tracks fields separately. Iterate over `&self.items`
directly and update `self.total` and `self.biggest` in the loop body: those are different fields, so
the borrows do not overlap. A method call cannot express "only these fields", which is why the helper
has to go (or change).

### Alternatives

Move the items out of `self` for the duration of the loop with `std::mem::take`, which leaves an
empty `Vec` behind. Now `self` is not borrowed by the loop, and `record` can stay as it was. Do not
forget to put the items back.

```rust
pub fn checkout(&mut self) {
    self.total = 0;
    self.biggest = None;
    let items = std::mem::take(&mut self.items);
    for item in &items {
        self.record(&item.name, item.price * item.qty);
    }
    self.items = items;
}
```

You can also turn `record` into a function that takes only the fields it changes, such as
`fn record(total: &mut u32, biggest: &mut Option<(String, u32)>, ...)`, and call it with
`&mut self.total` and `&mut self.biggest`.
