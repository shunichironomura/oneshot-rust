`From` を実装すれば、`Into` は自動で手に入ります。標準ライブラリに `impl<T, U: From<T>> Into<U> for T` があるからです。
`From<Celsius> for Fahrenheit` を書いた時点で `Celsius(-40.0).into()` が使えるのはこのためです。
さらに反射的な `impl<T> From<T> for T` もあるので、`Celsius` ははじめから `Into<Celsius>` です。
`is_freezing(Celsius(0.5))` や `average(vec![Celsius(10.0)])` は何も足さずに動きます。
自分で `impl From<Celsius> for Celsius` を書くと、実装の衝突（E0119）としてエラーになります。

このブランケット実装は一方向だけです。`T: Into<Celsius>` という境界しかないとき、`Celsius::from(t)` は
エラーになります（E0277、`Celsius: From<T>` を満たさない）。`t.into()` を呼び、変換先の型を
`let c: Celsius = t.into();` のように明示します。具体的な型では、変換先の型がさらに重要です。
`Fahrenheit(50.0).into().0` は E0282（型注釈が必要）になります。`Fahrenheit` は `Celsius` にも、
反射的な実装によって `Fahrenheit` にもなれるからです。なお、ジェネリックな `t` では `t.into().0` でも
たまたまコンパイルが通ります。コンパイラが知っている `t` の変換が、境界の `Into<Celsius>` だけだからです。
`warmest` では `Celsius::from(f)` と書けば迷いません。引数は `&Fahrenheit` ではなく `Fahrenheit` なので、
`|&f|` というパターンで受け取ります。

`f64` は `PartialOrd` ですが `Ord` ではありません（NaN は順序づけできません）。そのため、浮動小数点数にも
ニュータイプにも `Iterator::max` は使えず、コンパイルエラーになります。`f64::total_cmp` を渡した `max_by` か、
`reduce(f64::max)` を使います。`0.0` から `fold` すると、すべての測定値が 0 未満のときに答えを間違えます。

`average` では、`Celsius` が `Sum` を実装していないので、中の `f64` を足します。`into_iter()` は `Vec` を
消費するので、`len()` は先に読んでおき、割る前に `as` で `f64` にします。空のリストは別に扱います。
`0.0 / 0.0` は `None` ではなく NaN です。

### 別解

`warmest` は、華氏のまま最大値を求めてから、その 1 つだけを変換することもできます。クロージャは
`Fahrenheit(f).into()` を返し、変換先が `Celsius` であることは `warmest` の返り値の型からわかります。

```rust
pub fn warmest(readings: &[Fahrenheit]) -> Option<Celsius> {
    readings
        .iter()
        .map(|f| f.0)
        .reduce(f64::max)
        .map(|f| Fahrenheit(f).into())
}
```

ループを使った `average` と、変換先の型をトレイトに書いた `is_freezing` です。

```rust
pub fn is_freezing(t: impl Into<Celsius>) -> bool {
    Into::<Celsius>::into(t).0 <= 0.0
}

pub fn average<T: Into<Celsius>>(readings: Vec<T>) -> Option<Celsius> {
    let count = readings.len();
    let mut total = 0.0;
    for r in readings {
        let c: Celsius = r.into();
        total += c.0;
    }
    if count == 0 {
        None
    } else {
        Some(Celsius(total / count as f64))
    }
}
```
