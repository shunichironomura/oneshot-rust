pub struct Tree {
    pub children: Vec<Tree>,
}

/// The number of levels of the tree (1 for a single node).
pub async fn depth(t: &Tree) -> usize {
    let mut d = 0;
    for c in &t.children {
        d = d.max(depth(c).await);
    }
    d + 1
}
