関連型は実装の一部です。どのコンテナ型にも `Item` はちょうど 1 つ決まっていて、ジェネリックな関数では
`C: Container` という境界があれば、それを `C::Item`（`<C as Container>::Item` の略記）と書けます。
そこで、`total` をコンテナについてジェネリックにし、本体が必要とする境界は `C` ではなく、値の型で
ある `C::Item` に付けます。コンテナそのものを変換することはないからです。

本体は `C::Item` に 2 つのことを求めます。

- クロージャのパターン `|&x|` は、イテレータが返す `&C::Item` から値をコピーして取り出すので、
  要素は `Copy` でなければなりません。`Clone` では足りません（「cannot move out of a shared reference」）。
- `x.into()` には変換先の型が書かれていません。それを決めるのが境界 `C::Item: Into<i64>` です。
  `C::Item: Into<_>` という要求に合う where 節があると、コンパイラはそれを使って `i64` と推論します。
  境界 `i64: From<C::Item>` でも（`Into` のブランケット実装を通して）変換はできますが、`into` の
  変換先は決まらないままです。`sum()` からも決まりません。`i64` は `Sum<i64>` と `Sum<&i64>` の
  両方を実装しているからです。その結果、E0283「type annotations needed」になります。
  テンプレートの本体が `Bag` に対してさえコンパイルできないのも同じ理由です。`u8` はたくさんの型に
  変換できます。

### 別解

引数位置の `impl Trait` を使う場合、要素への境界は関連型境界（`Item: Bound`）で書けます。

```rust
pub fn total(c: &impl Container<Item: Copy + Into<i64>>) -> i64 {
    c.items().iter().map(|&x| x.into()).sum()
}
```

トレイトオブジェクトでも書けます。`dyn Container` では関連型を指定しなければならないので、
関数は要素の型 `T` についてだけジェネリックにします。`&Bag` は `&dyn Container<Item = u8>` に
型強制されます。

```rust
pub fn total<T: Copy + Into<i64>>(c: &dyn Container<Item = T>) -> i64 {
    c.items().iter().map(|&x| x.into()).sum()
}
```
