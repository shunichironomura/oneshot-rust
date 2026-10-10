`list` の先頭からノードを 1 つずつ外し、`reversed` の先頭につなぎ替えます。`node.next.take()` は
残りのリストを取り出して代わりに `None` を置くので、コピーや新たな確保なしにノードをつなぎ替えられます。

### 別解

`std::mem::replace` を使うと、新しい `next` を入れることと古い `next` を取り出すことを 1 行で書けます。

```rust
pub fn reverse(mut list: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut reversed = None;
    while let Some(mut node) = list {
        list = std::mem::replace(&mut node.next, reversed);
        reversed = Some(node);
    }
    reversed
}
```

再帰でも書けますが、リストの長さに比例してスタックが深くなります。
