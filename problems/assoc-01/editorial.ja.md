関連型は実装の一部です。どのコンテナ型にも `Item` はちょうど 1 つ決まっていて、ジェネリックな関数では
`C: Container` という境界があれば、それを `C::Item`（`<C as Container>::Item` の略記）と書けます。
そこで、どの関数もコンテナについてジェネリックにし、本体が必要とする境界は `C` ではなく、値の型で
ある `C::Item` に付けます。コンテナそのものを変換したり、比べたり、表示したりすることはないからです。

- `move_all` には型引数が **2 つ**必要です。2 つの引数に同じ `C` を使うと、`from` と `to` は同じ型で
  なければならず、`Stack<u8>` から `Bag` へ移す呼び出しがエラーになります。無関係な 2 つの型引数
  `A` と `B` にすると、今度は本体がエラーになります。`to.put(x)` は `B::Item` を受け取るのに、
  渡しているのは `A::Item` だからです。等式制約 `B: Container<Item = A::Item>` で、`B` が `A` と同じ
  型の値を保持することを表します（Rust に `where A::Item == B::Item` という節はありません）。
  ほかの境界は要りません。値は移動するだけなので、`Clone` でない `Book` でも動きます。
- `total` は `C::Item` に 2 つのことを求めます。クロージャのパターン `|&x|` は、イテレータが返す
  `&C::Item` から値をコピーして取り出すので、要素は `Copy` でなければなりません。`Clone` では
  足りません（「cannot move out of a shared reference」）。また、`x.into()` には変換先の型が
  書かれていません。それを決めるのが境界 `C::Item: Into<i64>` です。`C::Item: Into<_>` という
  要求に合う where 節があると、コンパイラはそれを使って `i64` と推論します。境界
  `i64: From<C::Item>` でも（`Into` のブランケット実装を通して）変換はできますが、`into` の
  変換先は決まらないままです。`sum()` からも決まりません。`i64` は `Sum<i64>` と `Sum<&i64>` の
  両方を実装しているからです。その結果、E0283「type annotations needed」になります。
  テンプレートの本体が `Bag` に対してさえコンパイルできないのも同じ理由です。`u8` はたくさんの型に
  変換できます。
- `largest` には `C::Item: Ord` が必要です。`Iterator::max` は全順序を要求するからです。
  `PartialOrd` では足りません。`f64` も `PartialOrd` を実装していますが、`NaN` はどの値とも
  比べられないので、`max` は最大値を決められないことがあります。返り値の型 `Option<&C::Item>` は
  コンテナの中を指す参照です。参照の引数は `c` だけなので、ライフタイムの省略規則によって返り値は
  `c` に結び付けられ、ライフタイムを書く必要はありません。
- `describe_all` には `C::Item: Display` が必要です。`to_string` は `ToString` のメソッドで、
  `Display` を実装する型はすべて `ToString` を実装しているので、`C::Item: ToString` でも動きます。
  `Debug` は誤りです。`{}` が使うのは `Display` で、`Book` は `Debug` を実装してさえいません。

### 別解

引数位置の `impl Trait` を使う場合、要素への境界は関連型境界（`Item: Bound`）で書けます。
このときコンテナ型には名前がないので、シグネチャで要素の型に触れる必要があるところでは、
型引数 `T` で名前を付けます。

```rust
use std::fmt::Display;

pub fn move_all<T>(from: &mut impl Container<Item = T>, to: &mut impl Container<Item = T>) -> usize
pub fn total(c: &impl Container<Item: Copy + Into<i64>>) -> i64
pub fn largest<T: Ord>(c: &impl Container<Item = T>) -> Option<&T>
pub fn describe_all(c: &impl Container<Item: Display>) -> Vec<String>
```

トレイトオブジェクトでも書けます。`dyn Container` では関連型を指定しなければならないので、
どの関数も要素の型 `T` についてだけジェネリックにします。`&Bag` は `&dyn Container<Item = u8>` に
型強制されます。メソッドは動的ディスパッチで呼ばれます。

```rust
pub fn move_all<T>(from: &mut dyn Container<Item = T>, to: &mut dyn Container<Item = T>) -> usize
pub fn total<T: Copy + Into<i64>>(c: &dyn Container<Item = T>) -> i64
pub fn largest<T: Ord>(c: &dyn Container<Item = T>) -> Option<&T>
pub fn describe_all<T: Display>(c: &dyn Container<Item = T>) -> Vec<String>
```

`move_all` では、共通の要素の型を 3 つ目の型引数で表すこともできます。

```rust
pub fn move_all<T, A, B>(from: &mut A, to: &mut B) -> usize
where
    A: Container<Item = T>,
    B: Container<Item = T>,
```
