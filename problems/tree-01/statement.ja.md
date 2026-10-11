`i32` の値を持つ二分探索木を実装してください。型は `template.rs` に定義済みです（変更しないでください）。

```rust
pub struct Node {
    pub value: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

pub struct Tree {
    root: Option<Box<Node>>,
}
```

`Tree` に次のメソッドを実装します。

- `insert(&mut self, value: i32) -> bool`：ノードより小さい値は左、大きい値は右に入れます。
  `value` がすでに木にあれば何もせず `false` を、なければ挿入して `true` を返します。
- `contains(&self, value: i32) -> bool`：`value` が木にあるかどうかを返します。
- `to_sorted_vec(&self) -> Vec<i32>`：すべての値を昇順に並べて返します。
- `height(&self) -> usize`：根から葉までの最も長い経路上のノード数を返します
  （空の木は `0`、ノード 1 つなら `1`）。

### 例

`5, 3, 8, 1, 4` の順に挿入すると下の木になります。`to_sorted_vec()` は `[1, 3, 4, 5, 8]`、
`height()` は `3` を返し、`contains(4)` は `true`、`contains(7)` は `false` です。

```text
    5
   / \
  3   8
 / \
1   4
```

空の木では `to_sorted_vec()` は `[]`、`height()` は `0` です。
