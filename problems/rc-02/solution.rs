use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Node {
    pub name: String,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

pub fn node(name: &str) -> Rc<Node> {
    Rc::new(Node {
        name: name.to_string(),
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(Vec::new()),
    })
}

pub fn adopt(parent: &Rc<Node>, child: &Rc<Node>) {
    let old = child.parent.replace(Rc::downgrade(parent));
    if let Some(old) = old.upgrade() {
        old.children.borrow_mut().retain(|c| !Rc::ptr_eq(c, child));
    }
    parent.children.borrow_mut().push(Rc::clone(child));
}

pub fn parent_name(n: &Node) -> Option<String> {
    n.parent.borrow().upgrade().map(|p| p.name.clone())
}

pub fn path(n: &Rc<Node>) -> String {
    let mut names = vec![n.name.clone()];
    let mut current = n.parent.borrow().upgrade();
    while let Some(p) = current {
        names.push(p.name.clone());
        current = p.parent.borrow().upgrade();
    }
    names.reverse();
    names.join("/")
}

pub fn child_names(n: &Node) -> Vec<String> {
    n.children.borrow().iter().map(|c| c.name.clone()).collect()
}
