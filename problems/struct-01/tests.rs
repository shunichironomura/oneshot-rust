use crate::judge::{case, check, check_eq};
use crate::user::Stack;

pub fn run() {
    case("sample-1", || {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        check_eq(s.pop(), Some(2))?;
        check_eq(s.pop(), Some(1))?;
        check_eq(s.pop(), None)
    });
    case("hidden-1", || {
        let s: Stack<String> = Stack::new();
        check(s.is_empty(), "new stack should be empty")?;
        check_eq(s.len(), 0)?;
        check_eq(s.peek(), None)
    });
    case("hidden-2", || {
        let mut s = Stack::new();
        s.push(String::from("a"));
        s.push(String::from("b"));
        check_eq(s.peek().map(|x| x.as_str()), Some("b"))?;
        check_eq(s.len(), 2)?;
        check(!s.is_empty(), "stack with items should not be empty")
    });
    case("hidden-3", || {
        let mut s = Stack::new();
        for i in 0..100 {
            s.push(i);
        }
        let popped: Vec<i32> = std::iter::from_fn(|| s.pop()).take(3).collect();
        check_eq(popped, vec![99, 98, 97])?;
        check_eq(s.len(), 97)
    });
}
