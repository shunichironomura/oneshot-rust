次のコードはコンパイルが通り、手元で実行すると動いているように見えます。
しかし**未定義動作（UB）**を含んでおり、Miri で実行すると UB として検出されます。

`Lookup` は名前から数値を引く表で、使われ方の統計も記録します。

```rust
impl Lookup {
    pub fn new(pairs: &[(&str, u32)]) -> Lookup;
    pub fn insert(&mut self, key: &str, value: u32);
    pub fn get(&self, key: &str) -> Option<u32>;
    pub fn hits(&self) -> u32;
    pub fn misses(&self) -> u32;
    pub fn total(&self) -> u32;
    pub fn computations(&self) -> u32;
}
```

- `new` は `pairs` から表を作ります。`pairs` のキーはすべて異なります。回数はすべて 0 から始まります。
- `insert` は `key` を値 `value` で追加します。`key` がすでに表にあれば、値を置き換えます。
- `get` は `key` に対応する値を返し、**ヒット**を 1 回数えます。そのキーがなければ `None` を返し、
  **ミス**を 1 回数えます。`hits` と `misses` はそれぞれの回数を返します。これらを変えるのは `get` だけです。
- `total` は表のすべての値の合計を返します（表が空なら `0`）。合計は `total` を初めて呼んだときに計算して
  キャッシュし、以降の呼び出しでは計算し直さずにキャッシュした合計を返します。`insert` はキャッシュした合計を
  捨てるので、次の `total` で再び計算します。`new` と `insert` が自分で合計を計算することはありません。
- `computations` は、これまでに合計を計算した回数を返します。
- どの合計も `u32` に収まるものとします。

公開されているシグネチャは変えないでください。テストでは、1 つの `Lookup` を指す複数の共有参照（`&Lookup`）を
同時に使って `get` と `total` を呼び、`Rc<Lookup>` 経由でも呼びます。`Lookup` の非公開フィールドは変更して
かまいません。

UB を取り除いて、正しく動くようにしてください（`unsafe` を使い続けてもかまいません）。

### 例

```rust
let mut lookup = Lookup::new(&[("a", 1), ("b", 2)]);
lookup.get("a"); // Some(1)、ヒット
lookup.get("z"); // None、ミス
lookup.total(); // 3（計算する）
lookup.total(); // 3（キャッシュ）
// hits() == 1, misses() == 1, computations() == 1

lookup.insert("c", 10);
lookup.total(); // 13（再び計算する）
// computations() == 2
```
