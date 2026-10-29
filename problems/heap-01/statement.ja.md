救急外来では、患者を来た順ではなく、症状の重さの順に診察します。待合リスト `Triage` を実装してください。

```rust
pub struct Triage { /* 自由に定義 */ }

impl Triage {
    pub fn new() -> Self;
    pub fn arrive(&mut self, name: &str, severity: u8);
    pub fn next(&mut self) -> Option<String>;
    pub fn peek(&self) -> Option<&str>;
    pub fn len(&self) -> usize;
}
```

- `arrive` は患者を待合リストに加えます。重症度 `severity` は `0` から `255` で、大きいほど緊急です。
- 次に診察するのは、重症度が最も高い患者です。重症度が同じ患者どうしでは、先に来た患者を先に診察します。
  同じ名前の患者が来ることもありますが、別の患者として扱います。
- `next` は次に診察する患者をリストから取り除き、その名前を返します。`peek` は取り除かずにその名前を返します。
  待っている患者がいなければ、どちらも `None` を返します。
- `len` は待っている患者の数を返します。

`Triage` のフィールドは自由に決めてかまいません。エディタに入っているフィールドは出発点にすぎません。
`std` のものは何を使ってもかまいません（たとえば `std::collections::BinaryHeap`）。

### 例

```rust
let mut t = Triage::new();
t.arrive("ann", 2);
t.arrive("bob", 5);
t.arrive("cid", 5);
t.arrive("dan", 1);
// t.peek() は Some("bob")、t.len() は 4 です。
// t.next() を 5 回呼ぶと Some("bob")、Some("cid")、Some("ann")、Some("dan")、None の順に返ります。
```
