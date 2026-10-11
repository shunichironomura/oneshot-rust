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
        let mut link = &mut self.root;
        while let Some(node) = link {
            if value == node.value {
                return false;
            }
            link = if value < node.value { &mut node.left } else { &mut node.right };
        }
        *link = Some(Box::new(Node { value, left: None, right: None }));
        true
    }

    pub fn contains(&self, value: i32) -> bool {
        let mut link = &self.root;
        while let Some(node) = link {
            if value == node.value {
                return true;
            }
            link = if value < node.value { &node.left } else { &node.right };
        }
        false
    }

    pub fn to_sorted_vec(&self) -> Vec<i32> {
        fn walk(link: &Option<Box<Node>>, out: &mut Vec<i32>) {
            if let Some(node) = link {
                walk(&node.left, out);
                out.push(node.value);
                walk(&node.right, out);
            }
        }
        let mut out = Vec::new();
        walk(&self.root, &mut out);
        out
    }

    pub fn height(&self) -> usize {
        fn depth(link: &Option<Box<Node>>) -> usize {
            match link {
                None => 0,
                Some(node) => 1 + depth(&node.left).max(depth(&node.right)),
            }
        }
        depth(&self.root)
    }
}
