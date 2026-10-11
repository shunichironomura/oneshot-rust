This code compiles, and it may even seem to work when you run it natively. But it contains
**undefined behavior (UB)**, and Miri detects it.

`Queue<T>` is a first-in, first-out queue built on a singly linked list. Besides the first node, it
keeps a raw pointer to the last node, so that `push` does not have to walk the list.

```rust
pub struct Queue<T> { /* private fields */ }

impl<T> Queue<T> {
    pub fn new() -> Self;
    pub fn push(&mut self, elem: T);
    pub fn pop(&mut self) -> Option<T>;
    pub fn peek(&self) -> Option<&T>;
    pub fn peek_back(&self) -> Option<&T>;
    pub fn len(&self) -> usize;
}
```

- `new` creates an empty queue.
- `push` adds `elem` at the back of the queue.
- `pop` removes the element at the front and returns it. It returns `None` if the queue is empty.
- `peek` returns the element at the front and `peek_back` the element at the back, without removing
  them. Both return `None` if the queue is empty. With a single element, both return that element.
- `len` returns the number of elements.
- A queue that has been emptied by `pop` can be used again.
- An element returned by `pop` belongs to the caller. When a queue is dropped, each element still in
  it is dropped exactly once.
- Queues are ordinary values: the tests move them, return them from functions and store queues in
  other queues (`Queue<Queue<String>>`).
- Undefined behavior and memory leaks are detected by Miri and fail the submission.

Remove the UB so that the queue works correctly. Do not change the public signatures. You may change
the private fields and `Node`, and you may keep using `unsafe` if you like. Keep the design, though:
each element is stored in its own node on the heap, the nodes are linked from front to back, and the
queue keeps a pointer to the last node, so that every operation takes constant time. Do not store
the elements in `Vec`, `VecDeque` or another std collection.

### Examples

```rust
let mut q = Queue::new();
q.push(String::from("a"));
q.push(String::from("b"));
q.push(String::from("c"));
// peek() is Some("a"), peek_back() is Some("c"), len() is 3
q.pop(); // Some("a")
q.pop(); // Some("b")
q.push(String::from("d"));
// peek() is Some("c"), peek_back() is Some("d"), len() is 2
q.pop(); // Some("c")
q.pop(); // Some("d")
q.pop(); // None
// peek() and peek_back() are None, len() is 0
```

```rust
let item = Rc::new(7);
let mut q = Queue::new();
for _ in 0..3 {
    q.push(Rc::clone(&item));
}
// Rc::strong_count(&item) is 4
let first = q.pop(); // Some(7)
drop(q); // drops the two clones still in the queue
// Rc::strong_count(&item) is 2 (`item` and `first`)
```
