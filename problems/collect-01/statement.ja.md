袋（バッグ、多重集合）は、各要素が何回追加されたかも覚えている集合のようなものです。
`Bag<T>` を、Rust のイテレータの仕組み（`collect`、`extend`、`for` ループ、`rev`）で
使えるようにしてください。

```rust
pub struct Bag<T> {
    counts: BTreeMap<T, usize>,
}

impl<T: Ord> Bag<T> {
    pub fn new() -> Self;
    pub fn count(&self, x: &T) -> usize;
    pub fn len(&self) -> usize;
}

impl<T: Ord> FromIterator<T> for Bag<T>
impl<T: Ord> Extend<T> for Bag<T>
impl<'a, T: Ord + Copy + 'a> Extend<&'a T> for Bag<T>

// for (x, n) in &bag
impl<'a, T> IntoIterator for &'a Bag<T> // Item = (&'a T, usize), IntoIter = Iter<'a, T>
// for x in bag
impl<T: Clone> IntoIterator for Bag<T> // Item = T, IntoIter = IntoIter<T>
```

これらはすべて、イテレータ型 `Iter` と `IntoIter`、およびその `Iterator` と `DoubleEndedIterator` の
実装とあわせて、本体が `todo!()` の状態でエディタに入っています。`Iter` と `IntoIter` のフィールドは
仮のものなので、必要なものに置き換えてください（この 2 つの構造体には境界を追加してもかまいません）。
それ以外のシグネチャは変えないでください。

- `new` は空の袋を返します。`count(&x)` は `x` と等しい要素が追加された回数です（一度もなければ
  `0`）。`len` は重複も含めた要素の総数です。
- `collect` と `extend` は受け取った要素をすべて追加します。`Vec` と同じように、`Copy` な要素への
  参照で `extend` することもできます（`bag.extend(&[1, 2, 2])`）。
- 借用した袋をイテレートすると、異なる要素ごとに 1 つずつ `(要素, 個数)` の組を、要素の昇順に
  返します。イテレートした後も袋は使えます。
- 所有権ごと袋をイテレートすると、各要素を個数の回数だけ、昇順に返します。袋は異なる要素ごとに
  値を 1 つしか持っていないので、残りはクローンするしかありませんが、必要以上にクローンしては
  いけません。
  - `into_iter()` を呼んだ時点では何もクローンしません。
  - イテレータを使い切ったときには、個数が `n` の要素はちょうど `n - 1` 回クローンされています
    （袋が持っていた値そのものを `n` 個のうちの 1 つとして返します）。
- どちらのイテレータも両端から取り出せます。`next_back` は末尾から（降順に）取り出し、`next` と
  `next_back` の呼び出しは混ぜてもかまいません。どちらの端から取り出しても各項目はちょうど 1 回ずつ
  返され、すべて返した後はどちらのメソッドも `None` を返します。

### 例

```rust
let mut b: Bag<char> = "hello".chars().collect();
// b.count(&'l') == 2, b.len() == 5
b.extend("lo".chars());
b.extend(&['x', 'x']);
for (c, n) in &b {
    // (&'e', 1), (&'h', 1), (&'l', 3), (&'o', 2), (&'x', 2) の順
}
b.into_iter().collect::<String>() // "ehlllooxx"
```

```rust
let b: Bag<i32> = (1..=3).chain(2..=3).collect();
// b.count(&3) == 2, b.len() == 5
(&b).into_iter().next_back() // Some((&3, 2))
b.into_iter().rev().collect::<Vec<_>>() // [3, 3, 2, 2, 1]
Bag::<i32>::new().into_iter().next() // None
```
