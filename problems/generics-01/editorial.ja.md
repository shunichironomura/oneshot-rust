`impl<X, Y> Point<X, Y>` のうち、`impl` の直後の `<X, Y>` は 2 つの型引数を**宣言**し、
`Point<X, Y>` はそれを**使って**、メソッドがどの型のものかを表します。テンプレートの `impl Point` は
何も宣言していないので、`new` の `X` と `Y` は未知の名前になり（「cannot find type `X` in this
scope」）、裸の `Point` には 2 つの型引数が足りません。3 つのブロックには、それぞれ別のヘッダが必要です。

- `new`、`mixup`、`swap`、`convert` はどの点にも使えるので、ブロックは境界なしの
  `impl<X, Y> Point<X, Y>` です。最初の 3 つの本体は値を移動しているだけで、`convert` は必要な条件を
  自分で書きます（後述）。`X: Clone` のような境界を付けると、ハーネスの `Grade` を受け付けなくなります。
- `mixup` の `other` は座標の型が別なので、メソッド自身の型引数を宣言します。
  `mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X, Y2>` です。結果は 2 組の型引数を混ぜたものに
  なります。`other: Self` にすると 2 つの点の型が同じでなければならず、
  `Point::new(5, 10.4).mixup(Point::new("Hello", 'c'))` がコンパイルできません。新しい名前は
  `X`、`Y` と別にする必要があります。`impl` がすでに `X` という名前を使っているので、
  `mixup<X, Y2>` はエラーになります。
- `swap` の返り値は `Self` ではなく `Point<Y, X>` です。このブロックの中で `Self` は `Point<X, Y>` で
  あり、`X` と `Y` がたまたま同じでない限り別の型です。
- `convert` も `X2` と `Y2` を宣言します。`.into()` の呼び出しには `X: Into<X2>` と `Y: Into<Y2>` が
  必要です。この境界にはメソッドの中にしかない `X2` と `Y2` が出てくるので、`impl` の行には書けず、
  メソッドの `where` 節に書きます。メソッドの `where` 節には `impl` の型引数への境界も書けて、
  その場合はそのメソッドだけに条件が付きます。ハーネスはターボフィッシュで
  `convert::<String, u32>()` と呼ぶので、`<X2, Y2>` の順番にも意味があります。
- `distance_from_origin` は `Point<f64, f64>` 専用なので、ブロックではその型を書き、何も宣言しません。
  `impl Point<f64, f64>` です。ジェネリックな `T` ではうまくいきません。`sqrt` はトレイトのメソッド
  ではなく浮動小数点数型に固有のメソッドで、`self.x * self.x` は `&self` から座標をコピーして
  取り出します。`impl<f64> Point<f64, f64>` には注意してください。これは本物の `f64` を隠す
  `f64` という名前の型**引数**を宣言するので、本体が「cannot multiply `f64` by `f64`」という
  不可解なエラーで失敗します。The Book の `Point<f32>` のように引数を 1 つにした `impl Point<f64>` も、
  `Point` の型引数は 2 つなのでエラーです。
- `larger` は `x` と `y` を比べて片方を整形するので、2 つの座標を同じ 1 つの型 `T` にし、
  `T: PartialOrd + Display` とします。`>=` は `PartialOrd` の、`{}` は `Display` の機能です。
  それ以上は要求しません。`Ord` にすると `f64` と `Grade` が、`Copy` を付けると `String` と `Grade` が、
  `Clone` を付けると `Grade` が使えなくなります。どれも不要です。`self.x >= self.y` は参照越しに比較し
  （`PartialOrd::ge(&self.x, &self.y)` を呼びます）、`format!` は引数を借用するからです。

`Point::new(3.0, 4.0).distance_from_origin()` のリテラルは、まだ `f32` にも `f64` にもなりえます。
`distance_from_origin` は `Point<f64, f64>` のブロックにしかないので、`f64` に決まります
（`Point<f32, f32>` のブロックも追加すると、この呼び出しはあいまいになります）。

### 別解

境界は `impl` の `where` 節にも書けます。

```rust
impl<T> Point<T, T>
where
    T: PartialOrd + Display,
{
    // larger はそのまま
}
```

`larger` は、互いに比較できる 2 つの異なる型も受け付けるようにできます。`PartialOrd<Y>` は
「`Y` と比較できる」という意味で、`T: PartialOrd` は `T: PartialOrd<T>` の略記です。

```rust
impl<X: PartialOrd<Y> + Display, Y: Display> Point<X, Y> {
    // larger はそのまま
}
```

この境界をメソッドの `where` 節に書けば、`larger` を最初のブロックの `convert` の隣に移すことも
できます。その場合、別のブロックは要りません。

```rust
    pub fn larger(&self) -> String
    where
        X: PartialOrd<Y> + Display,
        Y: Display,
    {
        if self.x >= self.y {
            format!("x = {}", self.x)
        } else {
            format!("y = {}", self.y)
        }
    }
```

`convert` は `Into` の代わりに `From` を要求しても書けます。標準ライブラリは `U: From<T>` なら
いつでも `Into<U> for T` を実装しているので、`.into()` はそのまま使えます。

```rust
    pub fn convert<X2, Y2>(self) -> Point<X2, Y2>
    where
        X2: From<X>,
        Y2: From<Y>,
```
