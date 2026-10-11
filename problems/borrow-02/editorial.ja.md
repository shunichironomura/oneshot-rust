`self.items()` が返すスライスは、`self.items` だけではなく **`self` 全体**を借用します。
呼び出し側から見えるのは `fn items(&self) -> &[Item]` というシグネチャだけだからです。
ループは終わるまでこの借用を持ち続けるので、ループの中で `&mut self` を取る `self.record(...)` を呼ぶとエラーになります。
ループの中で直接 `self.total += ...` と書いても、同じ理由でエラーになります。

一方、1 つの関数の中であれば、コンパイラはフィールドを別々に追跡します。`&self.items` を直接回し、
ループの中で `self.total` と `self.biggest` を更新すれば、別のフィールドなので借用は重なりません。
メソッド呼び出しでは「このフィールドだけ」を表せないので、ヘルパーはなくす（か形を変える）必要があります。

### 別解

`std::mem::take` で、ループの間だけ商品を `self` から取り出す方法です（代わりに空の `Vec` が残ります）。
ループが `self` を借用しなくなるので、`record` はそのまま使えます。最後に商品を戻すのを忘れないでください。

```rust
pub fn checkout(&mut self) {
    self.total = 0;
    self.biggest = None;
    let items = std::mem::take(&mut self.items);
    for item in &items {
        self.record(&item.name, item.price * item.qty);
    }
    self.items = items;
}
```

`record` を、変更するフィールドだけを受け取る関数（`fn record(total: &mut u32, biggest: &mut Option<(String, u32)>, ...)` など）に変え、
`&mut self.total` と `&mut self.biggest` を渡して呼ぶ方法もあります。
