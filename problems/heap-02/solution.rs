use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn k_smallest(v: &[i32], k: usize) -> Vec<i32> {
    // `Reverse` flips the order, so the max-heap pops the smallest value first.
    let mut heap: BinaryHeap<Reverse<i32>> = v.iter().map(|&x| Reverse(x)).collect();
    let mut smallest = Vec::new();
    while smallest.len() < k {
        match heap.pop() {
            Some(Reverse(x)) => smallest.push(x),
            None => break,
        }
    }
    smallest
}
