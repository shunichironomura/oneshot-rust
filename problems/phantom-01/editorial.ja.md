数値を左に置くための impl、`impl<U, K: Into<f64>> Mul<Length<U>> for K` は、**孤児ルール**
（orphan rule）に違反しています（E0210「type parameter `K` must be covered by another type when it
appears before the first local type」）。`Mul` は標準ライブラリのトレイトなので、その impl には
ローカルな型が必要です。さらに `impl<..> Mul<T1> for T0` の型は `T0`、`T1` の順に調べられ、
最初のローカルな型より前に、むき出しの型引数があってはいけません。ここでは `T0` が `K` そのもので、
どんな型でもありえます。ローカルなのは `T1 = Length<U>` だけです。このルールは、クレート同士が
衝突しないようにするためのものです。`Meters` という型を定義したクレートは `impl<T> Mul<T> for Meters` を
書くことが許されているので、もし上の impl も許されると、`Meters * Length<U>` には 2 つのクレートに
1 つずつ、2 つの impl があることになってしまいます。

そこで、数値型ごとに impl を書きます。

```rust
impl<U> Mul<Length<U>> for f64 {
    type Output = Length<U>;

    fn mul(self, l: Length<U>) -> Length<U> {
        l * self
    }
}
```

`f32`、`i32`、`u32` についても同じです。`Mul` も `f64` も標準ライブラリのものですが、この impl は
許されます。`T0 = f64` は型引数を含まず、`T1 = Length<U>` はローカルな型だからです（中の `U` は
`Length` に「覆われて」います）。`U` はジェネリックのままにしてください。`Length<Mm>` と `Length<Inch>`
だけの impl では、テスト独自の単位に対応できません。本体は `l * k` を使い回すだけです。
`perimeter` の `2 * (w + h)` は左から `i32` を掛けていますが、これで変更せずにコンパイルが通ります。

逆向きにはこうした注意はいりません。`impl<U, K: Into<f64>> Mul<K> for Length<U>` では `T0` が
ローカルな `Length<U>` なので、その後ろの型引数 `K` は問題ありません。`l * k` は 1 つの impl で
`K: Into<f64>` を満たすすべての型に対応できますが、`k * l` には型ごとの impl が必要です。
（ジェネリックな impl を消して `perimeter` を `(w + h) * 2` と書いてもコンパイルは通りますが、
テストでは `3 * a` や `2.5 * l` と書きます。）

左側の数値型の impl が複数あると、`2 * (w + h)` の `2` のようなリテラルは `i32` とも `u32` ともとれます。
コンパイラはこれを既定の型 `i32`（`2.5` なら `f64`）に決めますが、それは型検査の最後なので、
`(2 * l).value()` のように結果に直接メソッドを呼ぶと「type annotations needed」になります。
その場合は `2i32 * l` と書くか、先に結果に型を付けてください。

単位の型が `Debug`、`Clone`、`Copy`、`PartialEq` を derive しているのは、`Length<U>` の derive が
`U` にもそれぞれのトレイトを要求するからです。`U` が `PhantomData<U>` の中にしか現れなくても同じです。

### 別解

マクロで 4 つの impl を書く方法です。`f64::from(self)` はどの数値型も値を失わずに変換します。

```rust
macro_rules! scalar_times_length {
    ($($t:ty),*) => {
        $(
            impl<U> Mul<Length<U>> for $t {
                type Output = Length<U>;

                fn mul(self, l: Length<U>) -> Length<U> {
                    Length::new(f64::from(self) * l.value)
                }
            }
        )*
    };
}

scalar_times_length!(f64, f32, i32, u32);
```
