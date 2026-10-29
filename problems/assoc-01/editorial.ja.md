関連型は実装の一部です。どのコンテナ型にも `Item` はちょうど 1 つ決まっていて、ジェネリックな関数では
`C: Container` という境界があれば、それを `C::Item`（`<C as Container>::Item` の略記）と書けます。

`move_all` には型引数が **2 つ**必要です。2 つの引数に同じ `C` を使うと、`from` と `to` は同じ型で
なければならず、`Stack<u8>` から `Bag` へ移す呼び出しがエラーになります。無関係な 2 つの型引数
`A` と `B` にすると、今度は本体がエラーになります。`to.put(x)` は `B::Item` を受け取るのに、
渡しているのは `A::Item` だからです。等式制約 `B: Container<Item = A::Item>` で、`B` が `A` と同じ
型の値を保持することを表します（Rust に `where A::Item == B::Item` という節はありません）。
ほかの境界は要りません。値は移動するだけなので、`Clone` でない `Book` でも動きます。

### 別解

共通の要素の型を 3 つ目の型引数で表すこともできます。

```rust
pub fn move_all<T, A, B>(from: &mut A, to: &mut B) -> usize
where
    A: Container<Item = T>,
    B: Container<Item = T>,
{
    let mut n = 0;
    while let Some(x) = from.take_last() {
        to.put(x);
        n += 1;
    }
    n
}
```

引数位置の `impl Trait` を使う場合、コンテナ型には名前がないので、要素の型に型引数 `T` で名前を付けます。

```rust
pub fn move_all<T>(from: &mut impl Container<Item = T>, to: &mut impl Container<Item = T>) -> usize {
    let mut n = 0;
    while let Some(x) = from.take_last() {
        to.put(x);
        n += 1;
    }
    n
}
```

トレイトオブジェクトでも書けます。`dyn Container` では関連型を指定しなければならないので、
関数は要素の型 `T` についてだけジェネリックにします。`&mut Bag` は `&mut dyn Container<Item = u8>` に
型強制されます。メソッドは動的ディスパッチで呼ばれます。

```rust
pub fn move_all<T>(from: &mut dyn Container<Item = T>, to: &mut dyn Container<Item = T>) -> usize {
    let mut n = 0;
    while let Some(x) = from.take_last() {
        to.put(x);
        n += 1;
    }
    n
}
```
