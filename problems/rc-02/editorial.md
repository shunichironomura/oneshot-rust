Every `adopt` in the template creates a **reference cycle**: the parent's `children` holds an `Rc`
to the child, and the child's `parent` holds an `Rc` back to the parent. An `Rc` frees its value
only when its strong count drops to 0, and inside a cycle each node keeps the other's count above 0.
Once the caller drops its handles, nothing can reach the nodes any more, but nothing frees them
either, so Miri reports them as leaked when the program ends. The same cycle explains the wrong
counts (`Rc::strong_count(&root)` is 2) and why `root` is still on the path after `drop(root)`.

The fix is to make one direction of each link non-owning. The parent owns its children, so
`children` stays a `Vec<Rc<Node>>`, and the pointer back to the parent becomes a `Weak<Node>`:

- `Weak::new()` points to nothing. It takes the place of `None` for a node without a parent.
- `Rc::downgrade(parent)` creates a `Weak` to the parent. It raises the weak count, not the strong
  count, so a child does not keep its parent alive. Like `Rc::strong_count` and `Rc::ptr_eq`, it is
  an associated function rather than a method (so that it never hides a method of `Node`):
  `parent.downgrade()` does not compile.
- `upgrade()` turns a `Weak` back into an `Option<Rc<Node>>`: `Some` while the parent is alive,
  `None` once it has been freed. That is exactly what `parent_name` and `path` need. Since the
  result is an `Option` again, the template's `map` and `while let` stay as they were; only
  `.as_ref()` and `.clone()` on the parent become `.upgrade()`.

In `adopt`, `RefCell::replace` stores the new parent and returns the old `Weak` in one step. The old
parent may already have been freed; then `upgrade()` returns `None` and there is nothing to remove
the child from. `Rc::ptr_eq` compares nodes by identity, so a sibling with the same name is kept.

Walking up with a single variable does not compile:

```rust
let mut cur = Rc::clone(n);
while let Some(p) = cur.parent.borrow().upgrade() {
    names.push(p.name.clone());
    cur = p; // error[E0506]: cannot assign to `cur` because it is borrowed
}
```

The `Ref` returned by `borrow()` is a temporary of the `while let` condition and lives until the end
of the loop body, so `cur` is still borrowed when it is reassigned. The solution reads the next
parent in a statement of its own, whose temporaries are dropped at its `;`.

Making both directions `Weak` removes the leak too, but then nothing owns a child: a node that the
caller no longer holds would be freed even though it is still in the tree.

### Alternatives

Keep the shape of the template's `adopt`. `Weak` has no `take` method, but `RefCell::take` works for
any `T: Default`: it returns the old `Weak` and leaves `Weak::new()` in its place.

```rust
pub fn adopt(parent: &Rc<Node>, child: &Rc<Node>) {
    if let Some(old) = child.parent.take().upgrade() {
        old.children.borrow_mut().retain(|c| !Rc::ptr_eq(c, child));
    }
    parent.children.borrow_mut().push(Rc::clone(child));
    *child.parent.borrow_mut() = Rc::downgrade(parent);
}
```

`path` can also be written recursively. Each call upgrades one parent and builds the path above it
first.

```rust
pub fn path(n: &Rc<Node>) -> String {
    match n.parent.borrow().upgrade() {
        Some(p) => format!("{}/{}", path(&p), n.name),
        None => n.name.clone(),
    }
}
```
