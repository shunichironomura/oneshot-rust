`#[derive]` は、フィールドに本当に必要な境界を調べません。ジェネリックな構造体では
すべての型引数に境界を付けるので、`#[derive(Debug, Clone, Copy, PartialEq)]` が生成するのは
`impl<U: Copy> Copy for Length<U>` や `impl<U: PartialEq> PartialEq for Length<U>` などです。
`U` は `PhantomData<U>` の中にしか現れず、`PhantomData<U>` はどんな `U` でも `Copy`、`PartialEq`、
`Debug` ですが、それでも境界は付きます。ジェネリックな `perimeter<U>` と `longest<U>` の中では
`U` について何もわからないので、`Length<U>` は `Copy` になりません。
`w + w` は `w` をムーブしてからもう一度使い（E0382）、`copied()` は `Length<U>: Copy` を要求します（E0277）。
`Mm` は何も実装していないので、`Length<Mm>` でさえ `Copy` でも比較可能でもありません。
この `#[derive]` の制限は「imperfect derive」と呼ばれています。

`U` に境界を付けずに、4 つの impl を手で書けば解決します。

```rust
impl<U> Clone for Length<U> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<U> Copy for Length<U> {}

impl<U> PartialEq for Length<U> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<U> fmt::Debug for Length<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Length").field(&self.value).finish()
    }
}
```

`Copy` にはメソッドがありませんが、`Clone` の実装が必要です。`Length<U>` はどんな `U` でも `Copy` なので、
`clone` は `*self` を返すだけで済みます。`debug_tuple` はフィールドを `{:?}` で整形して `Length(...)` と
表示するので、`10.0` は `{}` のときのような `10` ではなく `10.0` になります。これらの impl があれば、
`perimeter`、`longest`、`to_mm` は変更しなくてもコンパイルが通ります。

エディタのコードのコンパイルが通るだけの修正では不十分です。

- `perimeter` と `longest` に `U: Copy` を付ける（さらに例がコンパイルできるよう `Mm` と `Inch` に
  トレイトを derive する）と、シグネチャが変わります。テストの `Length<Cubit>` は相変わらず `Copy` でも
  比較可能でも表示可能でもないので、テストがコンパイルできません。
- `Length::new(2.0 * (w.value + h.value))` のようにしてコピーを避けても、同じ問題が残ります。
  テスト自身が長さをコピーし、比較し、表示するからです。
- `Clone` を手で書いて `#[derive(Copy)]` を残すと、コンパイルが通りません。`Length<U>` が `Copy` なのは
  `U: Copy` のときだけになるので、`*self` で参照の先からムーブできないからです。
- フィールドを `PhantomData<fn() -> U>` などに変えても解決しません。derive が境界を付けるのは
  フィールドではなく型引数だからです。

### 別解

impl はほかの書き方もできます。`clone` で新しい長さを作り、`eq` で `value()` を呼び
（`Length<U>` は `Copy` なので `*self` がコピーされます）、`Debug` では `write!` を使う書き方です。
`debug_tuple` と違って `write!` では `{:#?}`（整形表示）が効きませんが、テストでは使いません。

```rust
impl<U> Clone for Length<U> {
    fn clone(&self) -> Self {
        Length::new(self.value)
    }
}

impl<U> Copy for Length<U> {}

impl<U> PartialEq for Length<U> {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl<U> fmt::Debug for Length<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Length({:?})", self.value)
    }
}
```
