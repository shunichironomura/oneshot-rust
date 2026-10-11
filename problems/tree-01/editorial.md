Every child slot is an `Option<Box<Node>>`, so the tree can be handled one slot at a time. In
`insert`, walk down with a cursor `link: &mut Option<Box<Node>>`. While it points at `Some(node)`,
move it to `&mut node.left` or `&mut node.right`; once it reaches an empty slot, write the new node
through it with `*link = Some(...)`. Holding a reference to the **slot** rather than to a node is
what lets you attach a child without special-casing the root.

`to_sorted_vec` and `height` follow the shape of the data: an in-order walk (left, self, right)
visits the values in ascending order, and the height of a slot is `0` for `None` and
`1 + max(left, right)` otherwise. Small helper functions that take `&Option<Box<Node>>` keep the
recursion simple, because the root and the children have the same type.

### Alternatives

`insert` can also be written recursively on the slot. The `match` on `link` borrows the node only
inside each arm, so the `None` arm can assign to `*link`.

```rust
fn insert_at(link: &mut Option<Box<Node>>, value: i32) -> bool {
    match link {
        None => {
            *link = Some(Box::new(Node { value, left: None, right: None }));
            true
        }
        Some(node) if value < node.value => insert_at(&mut node.left, value),
        Some(node) if value > node.value => insert_at(&mut node.right, value),
        Some(_) => false,
    }
}
```

Then `insert` is just `insert_at(&mut self.root, value)`. The recursion depth grows with the height
of the tree, which is fine for balanced trees but not for long chains.
