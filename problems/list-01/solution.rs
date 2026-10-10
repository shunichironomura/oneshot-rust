#[derive(Debug, PartialEq)]
pub struct Node {
    pub value: i32,
    pub next: Option<Box<Node>>,
}

pub fn reverse(mut list: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut reversed = None;
    while let Some(mut node) = list {
        list = node.next.take();
        node.next = reversed;
        reversed = Some(node);
    }
    reversed
}
