use std::pin::Pin;

/// Resolves to a greeting for `name`.
pub fn greet(name: String) -> impl Future<Output = String> {
    async { format!("Hello, {name}!") }
}

/// Awaits the futures one after another and adds up their outputs.
pub async fn sum_all(futs: Vec<Pin<Box<dyn Future<Output = u32>>>>) -> u32 {
    futs.into_iter().map(|f| f.await).sum()
}

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
