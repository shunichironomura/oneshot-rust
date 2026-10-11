次のコードはコンパイルが通り、手元で実行すると動いているように見えます。
しかし**未定義動作**（UB）を含んでおり、Miri で実行すると UB として検出されます。

`Lookup` は名前から数値を引く表で、使われ方の統計も記録します。

```rust
impl Lookup {
    pub fn new(pairs: &[(&str, u32)]) -> Lookup;
    pub fn insert(&mut self, key: &str, value: u32);
    pub fn get(&self, key: &str) -> Option<u32>;
    pub fn hits(&self) -> u32;
    pub fn misses(&self) -> u32;
    pub fn keys(&self) -> &[String];
    pub fn sorts(&self) -> u32;
}
```

- `new` は `pairs` から表を作ります。`pairs` のキーはすべて異なります。回数はすべて 0 から始まります。
- `insert` は `key` を値 `value` で追加します。`key` がすでに表にあれば、値を置き換えます。
- `get` は `key` に対応する値を返し、**ヒット**を 1 回数えます。そのキーがなければ `None` を返し、
  **ミス**を 1 回数えます。`hits` と `misses` はそれぞれの回数を返します。これらを変えるのは `get` だけです。
- `keys` は表のすべてのキーを、`<` による文字列の比較で昇順に並べて返します（バイトごとに比べるので、
  `"10" < "9"`、`"Z" < "a"` です）。表が空なら空のスライスを返します。
- この一覧は `keys` を初めて呼んだときに並べ替えてキャッシュし、以降の呼び出しでは並べ替え直さずに
  キャッシュした一覧を返します。`insert` は（キーがすでに表にあった場合も）キャッシュした一覧を捨てるので、
  次の `keys` で再び並べ替えます。`new` と `insert` が自分でキーを並べ替えることはありません。
- `sorts` は、これまでに一覧を並べ替えた回数を返します。

公開されているシグネチャは変えないでください。テストでは、1 つの `Lookup` を指す複数の共有参照（`&Lookup`）を
同時に使って `get` と `keys` を呼び、`Rc<Lookup>` 経由でも呼びます。また、`keys` が返したスライスを
使い続けながら、ほかのメソッドを呼びます。`Lookup` の非公開フィールドは変更してかまいません。

UB を取り除いて、正しく動くようにしてください（`unsafe` を使い続けてもかまいません）。

### 例

```rust
let mut lookup = Lookup::new(&[("b", 2), ("a", 1)]);
lookup.get("a"); // Some(1)、ヒット
lookup.get("z"); // None、ミス
lookup.keys(); // ["a", "b"]（並べ替える）
lookup.keys(); // ["a", "b"]（キャッシュ）
// hits() == 1、misses() == 1、sorts() == 1

lookup.insert("c", 10);
lookup.keys(); // ["a", "b", "c"]（再び並べ替える）
// sorts() == 2
```
