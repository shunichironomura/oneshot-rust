子を入れる場所はどれも `Option<Box<Node>>` なので、木は「場所」単位で扱えます。`insert` では
`link: &mut Option<Box<Node>>` というカーソルで下っていきます。`Some(node)` を指している間は
`&mut node.left` か `&mut node.right` に進め、空の場所に着いたら `*link = Some(...)` で新しいノードを書き込みます。
ノードではなく**場所**への参照を持つことで、根かどうかを場合分けせずに子をつなげます。

`to_sorted_vec` と `height` はデータの形どおりに書けます。左、自分、右の順に回る中間順の走査は
値を昇順に訪れます。高さは `None` なら `0`、そうでなければ `1 + max(左, 右)` です。
根も子も同じ型なので、`&Option<Box<Node>>` を受け取る補助関数を作ると再帰が素直に書けます。

### 別解

`insert` も場所に対する再帰で書けます。`link` に対する `match` はノードを各アームの中でだけ借用するので、
`None` のアームで `*link` に代入できます。

```rust
fn insert_at(link: &mut Option<Box<Node>>, value: i32) -> bool {
    match link {
        None => {
            *link = Some(Box::new(Node { value, left: None, right: None }));
            true
        }
        Some(node) if value < node.value => insert_at(&mut node.left, value),
        Some(node) if value > node.value => insert_at(&mut node.right, value),
        Some(_) => false,
    }
}
```

こうすれば `insert` は `insert_at(&mut self.root, value)` だけになります。再帰の深さは木の高さに比例するので、
釣り合った木なら問題ありませんが、長い一本道の木には向きません。
