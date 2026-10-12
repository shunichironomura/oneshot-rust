use std::collections::VecDeque;

pub fn josephus(n: usize, k: usize) -> Vec<usize> {
    // The front of the deque is the person who says 1 next.
    let mut circle: VecDeque<usize> = (1..=n).collect();
    let mut order = Vec::new();
    while !circle.is_empty() {
        // The k - 1 people who count before the k-th one move to the back, in order.
        // Only the remainder matters, and `rotate_left` panics if asked to rotate past `len`.
        circle.rotate_left((k - 1) % circle.len());
        if let Some(person) = circle.pop_front() {
            order.push(person);
        }
    }
    order
}
