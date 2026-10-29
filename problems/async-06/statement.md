The code below does not compile. Fix it.

The test harness runs the async function below with its own small executor (only `std` is
available, there is no async runtime).

```rust
pub struct Tree {
    pub children: Vec<Tree>,
}

pub async fn depth(t: &Tree) -> usize;
```

- `depth(t)` resolves to the number of levels of the tree: `1` for a node without children,
  otherwise 1 plus the largest depth among its children.

Do not change the signature or the `Tree` struct. Keep the recursion in `depth` asynchronous: the
depth of each child must still come from an awaited recursive call.

### Examples

- A root with two children, the first of which has one child of its own, has depth `3`.
