Take the nodes off the front of `list` one at a time and push them onto the front of `reversed`.
`node.next.take()` moves the rest of the list out and leaves `None` in its place, so `node` can be
relinked without copying or allocating anything.

### Alternatives

`std::mem::replace` swaps in the new `next` and takes out the old one in a single step.

```rust
pub fn reverse(mut list: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut reversed = None;
    while let Some(mut node) = list {
        list = std::mem::replace(&mut node.next, reversed);
        reversed = Some(node);
    }
    reversed
}
```

A recursive version also works, but its stack depth grows with the length of the list.
