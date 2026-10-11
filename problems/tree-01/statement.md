Implement a binary search tree of `i32` values. The types are defined in `template.rs`; do not
change them.

```rust
pub struct Node {
    pub value: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

pub struct Tree {
    root: Option<Box<Node>>,
}
```

Implement these methods of `Tree`:

- `insert(&mut self, value: i32) -> bool`: values smaller than a node go to its left, larger ones to
  its right. If `value` is already in the tree, leave the tree unchanged and return `false`;
  otherwise insert it and return `true`.
- `contains(&self, value: i32) -> bool`: whether `value` is in the tree.
- `to_sorted_vec(&self) -> Vec<i32>`: all values in ascending order.
- `height(&self) -> usize`: the number of nodes on the longest path from the root down to a leaf
  (`0` for an empty tree, `1` for a single node).

### Example

Inserting `5, 3, 8, 1, 4` in this order gives the tree below. `to_sorted_vec()` returns
`[1, 3, 4, 5, 8]`, `height()` returns `3`, `contains(4)` is `true` and `contains(7)` is `false`.

```text
    5
   / \
  3   8
 / \
1   4
```

For an empty tree, `to_sorted_vec()` is `[]` and `height()` is `0`.
