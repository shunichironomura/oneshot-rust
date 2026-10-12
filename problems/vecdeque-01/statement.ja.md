テキストエディタは、取り消しができるように操作を覚えていますが、覚えておくのは最新の `capacity` 個だけです。
この履歴 `History` を実装してください。

```rust
pub struct History { /* 自由に定義 */ }

impl History {
    pub fn new(capacity: usize) -> History;
    pub fn record(&mut self, action: &str);
    pub fn undo(&mut self) -> Option<String>;
    pub fn recent(&self, n: usize) -> Vec<&str>;
    pub fn oldest(&self) -> Option<&str>;
    pub fn len(&self) -> usize;
}
```

- `new` は、操作を最大 `capacity` 個まで覚える空の履歴を返します。`capacity` は `0` のこともあります。
- `record` は `action` を最新の操作として加えます。その結果、操作が `capacity` 個より多くなったら、
  最も古い操作を捨てます。したがって、`capacity` が `0` の履歴は何も覚えません。
  捨てた操作が戻ってくることはありません。
- 同じ文字列が何回も記録されることがあります。`record` 1 回ごとに別の操作として扱います。
- `undo` は最新の操作を履歴から取り除いて返します。履歴が空なら `None` を返します。
- `recent` は新しいほうから `n` 個の操作を、新しい順に返します。覚えている操作が `n` 個より少なければ、
  すべてを返します。`n` は `0` から `usize::MAX` までのどの値もありえます。
- `oldest` は覚えている操作のうち最も古いものを返します。履歴が空なら `None` を返します。
- `len` は覚えている操作の数を返します。

`History` のフィールドは自由に決めてかまいません。エディタに入っているフィールドは出発点にすぎません。
`std` のものは何を使ってもかまいません（たとえば、両端で要素を出し入れできるキュー
`std::collections::VecDeque`）。

### 例

```rust
let mut h = History::new(3);
h.record("type");
h.record("bold");
h.record("paste");
h.record("delete");
// "type" は捨てられています。h.len() は 3、h.oldest() は Some("bold") です。
// h.recent(2) は ["delete", "paste"]、h.recent(10) は ["delete", "paste", "bold"] です。
// h.undo() は Some("delete") を返し、その後の h.recent(10) は ["paste", "bold"] です。
```
