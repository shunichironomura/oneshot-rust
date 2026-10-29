pub struct Tree {
    pub children: Vec<Tree>,
}

/// The number of levels of the tree (1 for a single node).
pub async fn depth(t: &Tree) -> usize {
    let mut d = 0;
    for c in &t.children {
        // The inner future lives on the heap, so this future only holds a pointer to it.
        d = d.max(Box::pin(depth(c)).await);
    }
    d + 1
}
