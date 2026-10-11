use std::rc::Rc;

use crate::judge::{case, check, check_eq};
use crate::user::{adopt, child_names, node, parent_name, path};

fn name(s: &str) -> Option<String> {
    Some(s.to_string())
}

const NO_NAMES: [&str; 0] = [];

pub fn run() {
    case("sample-1", || {
        let root = node("root");
        let a = node("a");
        let b = node("b");
        adopt(&root, &a);
        adopt(&a, &b);
        check_eq(path(&b), "root/a/b")?;
        check_eq(parent_name(&a), name("root"))?;
        check_eq(child_names(&root), ["a"])?;
        check_eq(
            (Rc::strong_count(&root), Rc::weak_count(&root), Rc::strong_count(&a)),
            (1, 1, 2),
        )
    });
    case("sample-2", || {
        let root = node("root");
        let a = node("a");
        let b = node("b");
        adopt(&root, &a);
        adopt(&a, &b);
        drop(root);
        check_eq(path(&b), "a/b")?;
        check_eq(parent_name(&a), None)
    });
    case("hidden-1", || {
        // Several children keep their order, and each one is counted once.
        let mom = node("mom");
        let kids = [node("x"), node("y"), node("z")];
        for kid in &kids {
            adopt(&mom, kid);
        }
        check_eq(child_names(&mom), ["x", "y", "z"])?;
        check_eq(child_names(&kids[1]), NO_NAMES)?;
        check_eq((path(&mom), parent_name(&mom)), ("mom".to_string(), None))?;
        check_eq((Rc::strong_count(&mom), Rc::weak_count(&mom)), (1, 3))?;
        check_eq((Rc::strong_count(&kids[2]), Rc::weak_count(&kids[2])), (2, 0))
    });
    case("hidden-2", || {
        // A node whose handle the caller dropped lives on as long as its parent does.
        let root = node("root");
        let a = node("a");
        let b = node("b");
        adopt(&root, &a);
        adopt(&a, &b);
        let weak_a = Rc::downgrade(&a);
        drop(a);
        check_eq(path(&b), "root/a/b")?;
        check_eq(Rc::strong_count(&b), 2)?;
        drop(root);
        check(weak_a.upgrade().is_none(), "a should be freed together with root")?;
        check_eq(path(&b), "b")?;
        check_eq(Rc::strong_count(&b), 1)
    });
    case("hidden-3", || {
        // Moving a child to another parent, back again, and to the end of the same parent.
        let root = node("root");
        let other = node("other");
        let a = node("a");
        let b = node("b");
        adopt(&root, &a);
        adopt(&root, &b);
        adopt(&other, &a);
        check_eq(child_names(&root), ["b"])?;
        check_eq(path(&a), "other/a")?;
        check_eq((Rc::weak_count(&root), Rc::weak_count(&other), Rc::strong_count(&a)), (1, 1, 2))?;
        adopt(&root, &a);
        check_eq(child_names(&root), ["b", "a"])?;
        check_eq(child_names(&other), NO_NAMES)?;
        adopt(&root, &b);
        check_eq(child_names(&root), ["a", "b"])?;
        check_eq(
            (Rc::weak_count(&root), Rc::weak_count(&other), Rc::strong_count(&a), Rc::strong_count(&b)),
            (2, 0, 2, 2),
        )
    });
    case("hidden-4", || {
        // A child whose parent was freed can be adopted again, and nodes with the same name are
        // still different nodes.
        let c = node("twin");
        let twin = node("twin");
        {
            let p = node("p");
            adopt(&p, &c);
        }
        check_eq(parent_name(&c), None)?;
        check_eq(Rc::strong_count(&c), 1)?;
        let q = node("q");
        adopt(&q, &twin);
        adopt(&q, &c);
        check_eq(child_names(&q), ["twin", "twin"])?;
        let r = node("r");
        adopt(&r, &c);
        check_eq(child_names(&q), ["twin"])?;
        check_eq(parent_name(&twin), name("q"))?;
        check_eq((Rc::strong_count(&c), Rc::strong_count(&twin), Rc::weak_count(&q)), (2, 2, 1))
    });
    case("hidden-5", || {
        // A chain held only at both ends, then only at the top, then not at all.
        const NAMES: [&str; 6] = ["n0", "n1", "n2", "n3", "n4", "n5"];
        let top = node(NAMES[0]);
        let mut last = Rc::clone(&top);
        for name in &NAMES[1..] {
            let next = node(name);
            adopt(&last, &next);
            last = next;
        }
        check_eq((Rc::strong_count(&top), Rc::weak_count(&top), Rc::strong_count(&last)), (1, 1, 2))?;
        let leaf = Rc::downgrade(&last);
        drop(last);
        check_eq(leaf.upgrade().map(|n| path(&n)), Some("n0/n1/n2/n3/n4/n5".to_string()))?;
        drop(top);
        check(leaf.upgrade().is_none(), "the whole chain should be freed")
    });
}
