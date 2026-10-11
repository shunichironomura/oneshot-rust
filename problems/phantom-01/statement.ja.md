次のコードはコンパイルが通りません。**エラーメッセージを見ずに**、どこで怒られるかを予想して直してください。

`Length<U>` は、単位を型に含めた長さです。`Length<Mm>` と `Length<Inch>` は別の型なので、
ミリメートルとインチをうっかり混ぜることはできません。単位の型はデータを持たず、
`Length<U>` が保持するのは `f64` とサイズ 0 の `PhantomData<U>` だけです。

```rust
pub struct Mm;
pub struct Inch;

pub struct Length<U> {
    value: f64,
    unit: PhantomData<U>,
}

impl<U> Length<U> {
    pub fn new(value: f64) -> Self;
    pub fn value(self) -> f64;
}

impl<U> Add for Length<U> {
    type Output = Length<U>;
    // ...
}

pub fn perimeter<U>(w: Length<U>, h: Length<U>) -> Length<U>
pub fn longest<U>(items: &[Length<U>]) -> Option<Length<U>>
pub fn to_mm(l: Length<Inch>) -> Length<Mm>
```

- `new` は指定した値の長さを作り、`value` はその値を返します。
- `a + b` は値を足します。結果の単位は `a`、`b` と同じです。
- `perimeter(w, h)` は `w` × `h` の長方形の周の長さ `w + w + h + h` を返します。
- `longest(items)` は値が最大の要素のコピーを返します。`items` が空なら `None` です。
  スライス自体は変更しません。最大の値を持つ要素が複数あるときは、どれを返しても構いません。
  値が NaN になることはありません。
- `to_mm(l)` はインチをミリメートルに変換します。値を `25.4` 倍します。
- 長さは普通の数値と同じように扱えます。
  - `Length<U>` は `Copy` と `Clone` を実装します。テストでは、値渡しした後の長さも使い続けます。
  - 同じ単位の 2 つの長さは、値が `f64` として等しいときに `==` で等しくなります。
  - `{:?}` では `Length(<v>)` と表示します。`<v>` は値を `{:?}` で表示したものです
    （例：`Length(2.5)`、`Length(10.0)`）。
- これらはすべて、**どの**単位の型 `U` についても成り立たなければなりません。テストでは `Mm` と
  `Inch` のほかに、何も derive せず、トレイトを 1 つも実装していない独自の単位（`struct Cubit;` や
  `enum Furlongs {}` など）を定義して使います。`U` について何も知らないジェネリックなコードでも
  長さを使います。

公開されているシグネチャは変えないでください。上の規則を守っていれば、それ以外は変更して構いません。

### 例

```rust
let a = Length::<Mm>::new(2.0);
let p = perimeter(a, Length::new(3.0));
p == Length::new(10.0)  // true
a.value()               // 2.0（`a` はまだ使える）
format!("{p:?}")        // "Length(10.0)"

to_mm(Length::<Inch>::new(2.0)).value()  // 50.8
longest(&[Length::<Inch>::new(1.5), Length::new(4.0), Length::new(2.0)])  // Some(Length(4.0))
```
