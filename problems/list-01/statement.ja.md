単方向連結リストを反転してください。ノードの型は次のとおりです（`template.rs` に定義済み。変更しないでください）。

```rust
#[derive(Debug, PartialEq)]
pub struct Node {
    pub value: i32,
    pub next: Option<Box<Node>>,
}

pub fn reverse(list: Option<Box<Node>>) -> Option<Box<Node>>
```

ノードを新しく確保し直すのではなく、受け取ったノードをつなぎ替えて反転するのが理想です
（ただし判定はリストの中身だけを見ます）。

### 例

`1 -> 2 -> 3` は `3 -> 2 -> 1` になります。空リスト（`None`）は `None` のままです。
