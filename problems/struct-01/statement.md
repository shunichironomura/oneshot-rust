Implement a stack `Stack<T>` that can hold values of any type.

```rust
pub struct Stack<T> { /* up to you */ }

impl<T> Stack<T> {
    pub fn new() -> Self;
    pub fn push(&mut self, value: T);
    pub fn pop(&mut self) -> Option<T>;
    pub fn peek(&self) -> Option<&T>;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}
```

- `pop` removes and returns the most recently pushed value, or `None` if the stack is empty.
- `peek` returns a reference to the most recently pushed value without removing it.
