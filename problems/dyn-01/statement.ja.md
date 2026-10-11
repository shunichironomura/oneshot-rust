次のコードはコンパイルが通りません。**エラーメッセージを見ずに**、どこで怒られるかを予想して直してください。

`Unit` はゲームのパーティーのメンバーを表すトレイトで、`Knight` と `Archer` が実装しています。
`total_power` と `strongest` は、種類の混ざったパーティーを `&[Box<dyn Unit>]` として受け取り、
戦力の合計と、いちばん強いメンバーの名前（同じ戦力なら先にいるほう、パーティーが空なら `None`）を返します。

```rust
pub fn total_power(party: &[Box<dyn Unit>]) -> u32
pub fn strongest(party: &[Box<dyn Unit>]) -> Option<String>
```

ハーネスは `Knight` や `Archer` の値に対して `promoted` と `beats` も直接呼ぶので、
これらのメソッドはそこで使えるようにしておいてください。`total_power` と `strongest` のシグネチャは変えないでください。

### 例

`Knight { level: 2 }`（戦力 20）と `Archer { level: 3 }`（戦力 21）のパーティーに対して、`total_power` は `41` を、
`strongest` は `Some("archer(lv3)")` を返します。`Knight { level: 1 }.promoted()` はレベル 2 の騎士で、
レベル 2 の弓兵に `beats` します（勝ちます）。
