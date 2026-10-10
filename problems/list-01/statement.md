Reverse a singly linked list. The node type is defined in `template.rs`; do not change it.

```rust
#[derive(Debug, PartialEq)]
pub struct Node {
    pub value: i32,
    pub next: Option<Box<Node>>,
}

pub fn reverse(list: Option<Box<Node>>) -> Option<Box<Node>>
```

Ideally, relink the nodes you receive instead of allocating new ones (the judge only checks the
contents, though).

### Example

`1 -> 2 -> 3` becomes `3 -> 2 -> 1`. An empty list (`None`) stays `None`.
