This code compiles, and the trees it builds have the right shape. But Miri reports a **memory leak**
at the end of the run.

`Node` is a person in a family tree (it is already in the editor). The tree is built and read with
these functions:

```rust
pub struct Node {
    pub name: String,
    // private fields
}

pub fn node(name: &str) -> Rc<Node>
pub fn adopt(parent: &Rc<Node>, child: &Rc<Node>)
pub fn parent_name(n: &Node) -> Option<String>
pub fn path(n: &Rc<Node>) -> String
pub fn child_names(n: &Node) -> Vec<String>
```

- `node` creates a node with the given name, with no parent and no children. Different nodes may
  have the same name.
- `adopt` makes `child` the last child of `parent`. If `child` already has a parent (even if it is
  `parent` itself), it is first removed from that parent's children. `adopt` is never asked to make
  a node its own ancestor.
- A parent owns its children: a node stays alive while the caller holds an `Rc` to it **or** while
  it is a child of a living node. A child does not keep its parent alive. When a node is freed, its
  children that are still alive no longer have a parent.
- `parent_name` returns the name of the parent of `n`, or `None` if `n` has no parent.
- `path` follows the parents of `n` upward as far as they go, and joins the names from the topmost
  one down to `n` with `/`. For a node without a parent, it is just its name.
- `child_names` returns the names of the children of `n`, from the first to the last.
- `Rc::strong_count(&x)` is the number of `Rc` handles to `x` that the caller holds, plus 1 if `x`
  has a parent. `Rc::weak_count(&x)` is the number of `Weak` handles to `x` that the caller holds,
  plus the number of children of `x`.

Fix the leak, so that every node is freed as soon as the rules above say it is no longer alive. Do
not change the public signatures or the public field `name`. You may change the private fields of
`Node`.

### Example

```rust
let root = node("root");
let a = node("a");
let b = node("b");
adopt(&root, &a);
adopt(&a, &b);
// path(&b) == "root/a/b", parent_name(&a) == Some("root"), child_names(&root) == ["a"]
// Rc::strong_count(&root) == 1, Rc::weak_count(&root) == 1, Rc::strong_count(&a) == 2

drop(root);
// path(&b) == "a/b", parent_name(&a) == None
```
