次のコードはコンパイルが通りません。直してください。

`Cart` は商品を持っています。`checkout` は次の 2 つを最初から計算し直します。

- `total()`：全商品の `price * qty` の合計。
- `biggest()`：小計 `price * qty` が最大の商品の名前。同じ小計なら先にある商品を選びます。カートが空なら `None` です。

```rust
impl Cart {
    pub fn new(items: Vec<Item>) -> Self;
    pub fn checkout(&mut self);
    pub fn total(&self) -> u32;
    pub fn biggest(&self) -> Option<&str>;
}
```

公開されているシグネチャは変えないでください。非公開のヘルパーは変更・削除して構いません。

### 例

りんご（120 × 3）、メロン（500 × 1）、お茶（150 × 2）の場合、`checkout()` の後の合計は `1160`、最大は `Some("melon")` です。
