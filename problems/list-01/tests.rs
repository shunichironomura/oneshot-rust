use crate::judge::{case, check_eq};
use crate::user::{reverse, Node};

fn from_slice(values: &[i32]) -> Option<Box<Node>> {
    values
        .iter()
        .rev()
        .fold(None, |next, &value| Some(Box::new(Node { value, next })))
}

fn to_vec(mut list: &Option<Box<Node>>) -> Vec<i32> {
    let mut out = Vec::new();
    while let Some(node) = list {
        out.push(node.value);
        list = &node.next;
    }
    out
}

pub fn run() {
    case("sample-1", || check_eq(to_vec(&reverse(from_slice(&[1, 2, 3]))), vec![3, 2, 1]));
    case("sample-2", || check_eq(reverse(None), None));
    case("hidden-1", || check_eq(to_vec(&reverse(from_slice(&[42]))), vec![42]));
    case("hidden-2", || {
        let values: Vec<i32> = (0..50).collect();
        let expected: Vec<i32> = (0..50).rev().collect();
        check_eq(to_vec(&reverse(from_slice(&values))), expected)
    });
}
