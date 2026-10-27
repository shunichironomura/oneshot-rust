次のコードはコンパイルが通りません。直してください。

`Length<U>` は、単位を型に含めた長さです。`Length<Mm>` と `Length<Inch>` は別の型なので、
ミリメートルとインチをうっかり混ぜることはできません。単位の型はデータを持たず、
`Length<U>` が保持するのは `f64` とサイズ 0 の `PhantomData<U>` だけです。長さには普通の算術演算子が使え、
数値との掛け算はどちら側からでもできます。

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mm;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Inch;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Length<U> {
    value: f64,
    unit: PhantomData<U>,
}

impl<U> Length<U> {
    pub fn new(value: f64) -> Self;
    pub fn value(self) -> f64;
}

pub fn perimeter<U>(w: Length<U>, h: Length<U>) -> Length<U>
pub fn to_mm(l: Length<Inch>) -> Length<Mm>
```

- `new` は指定した値の長さを作り、`value` はその値を返します。同じ単位の 2 つの長さは、
  値が等しいときに `==` で等しくなります。
- 同じ単位の 2 つの長さ `a`、`b` について、`a + b` は値の和を値とする長さで、`a / b` は値の商です。
  商は単なる `f64` です（`a` の中に `b` がいくつ入るか）。
- `l * k` と `k * l` は、長さ `l` の値に、数値 `k` を `f64` に変換して掛けます。結果は同じ単位の
  長さです。`k` が右側のときは、`Into` で `f64` に変換できる数値型ならどれでも構いません（`u8` や
  `i16` など）。`k` が左側のときは、テストでは `f64`、`f32`、`i32`、`u32` を使います。
- `perimeter(w, h)` は `w` × `h` の長方形の周の長さ `2 * (w + h)` を返します。
- `to_mm(l)` はインチをミリメートルに変換します。値を `25.4` 倍します。
- これらはすべて、**どの**単位の型 `U` についても成り立たなければなりません。テストでは `Mm` と
  `Inch` のほかに、同じ 4 つのトレイトを derive した独自の単位（`struct Cubit;` や `enum Furlongs {}` など）を
  定義して使います。`U` について何も知らないジェネリックなコードでも長さを使います。

型と、`new`、`value`、`perimeter`、`to_mm` のシグネチャは変えないでください。上の規則を守っていれば、
それ以外は変更して構いません。

### 例

```rust
let a = Length::<Mm>::new(2.0);
a + a                           // Length::new(4.0)
a * 3                           // Length::new(6.0)
3 * a                           // Length::new(6.0)
perimeter(a, Length::new(3.0))  // Length::new(10.0)
a.value()                       // 2.0（`a` はまだ使える）

let l = Length::<Inch>::new(2.0);
2.5 * l                                    // Length::new(5.0)
to_mm(l).value()                           // 50.8
Length::<Mm>::new(6.0) / Length::new(1.5)  // 4.0
```
