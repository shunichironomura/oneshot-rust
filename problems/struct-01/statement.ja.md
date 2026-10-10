任意の型の値を積めるスタック `Stack<T>` を実装してください。

```rust
pub struct Stack<T> { /* 自由に定義 */ }

impl<T> Stack<T> {
    pub fn new() -> Self;
    pub fn push(&mut self, value: T);
    pub fn pop(&mut self) -> Option<T>;
    pub fn peek(&self) -> Option<&T>;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}
```

- `pop` は最後に積んだ値を取り出します。空なら `None` です。
- `peek` は最後に積んだ値への参照を返します（取り出しません）。
