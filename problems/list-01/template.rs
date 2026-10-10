#[derive(Debug, PartialEq)]
pub struct Node {
    pub value: i32,
    pub next: Option<Box<Node>>,
}

pub fn reverse(list: Option<Box<Node>>) -> Option<Box<Node>> {
    todo!()
}
