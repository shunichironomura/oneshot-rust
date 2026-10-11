#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

#[derive(Debug, Default)]
pub struct Tree {
    root: Option<Box<Node>>,
}

impl Tree {
    pub fn new() -> Tree {
        Tree { root: None }
    }

    pub fn insert(&mut self, value: i32) -> bool {
        todo!()
    }

    pub fn contains(&self, value: i32) -> bool {
        todo!()
    }

    pub fn to_sorted_vec(&self) -> Vec<i32> {
        todo!()
    }

    pub fn height(&self) -> usize {
        todo!()
    }
}
