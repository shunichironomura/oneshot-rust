`f64` は `PartialOrd` を実装していますが、`Ord` は実装していません。NaN はどの値（自分自身も含む）と比べても、
小さくも等しくも大きくもないからです。`Iterator::max`、`Iterator::min`、`slice::sort` は `Ord` を要求するので、
`f64` の値に対して呼ぶとコンパイルが通りません（E0277）。よくある回避策の `a.partial_cmp(b).unwrap()` は
コンパイルは通りますが、NaN を比べた時点でパニックします。

模範解答では、まず `filter(|x| !x.is_nan())` で NaN の測定値を取り除きます。残りは `f64::total_cmp` で
並べます。これはすべての `f64` の値に対する全順序で、`max_by`、`min_by`、`sort_by` にそのまま渡せます。
`-∞` と `∞` は期待どおりの位置に並びます。`total_cmp` は NaN にも順序を付ける（正の NaN は `∞` の後、
負の NaN は `-∞` の前）ので、やはりフィルタは必要です。フィルタがないと、正の NaN が最大値に、
負の NaN が最小値になってしまいます。

`spread` は最大値と最小値を再利用します。測定値が 1 つもなければ、`?` で早めに `None` を返します。

ループを自分で書く場合、最大値の初期値を `0.0` にすると、測定値がすべて負のときに間違えます。
初期値を `f64::MIN` にすると、`-∞` の測定値が 1 つだけのときに間違えます。初期値を `f64::NEG_INFINITY` に
すると、「測定値がない」と「`-∞` の測定値が 1 つだけある」の区別がつかなくなります。
途中の最大値を `Option<f64>` で持てば、これらはすべて避けられます。

### 別解

`f64::max` と `f64::min` は NaN を無視します。片方の引数が NaN なら、もう片方を返します。
そのため、「まだ値がない」ことを表す NaN から畳み込むと最大値が求まり、結果が NaN になるのは
測定値が 1 つもないときだけです。

```rust
pub fn max_reading(v: &[f64]) -> Option<f64> {
    let max = v.iter().copied().fold(f64::NAN, f64::max);
    if max.is_nan() { None } else { Some(max) }
}
```

NaN の測定値を取り除いた後なら、`partial_cmp` は必ず `Some` を返すので、`unwrap` しても安全です。

```rust
pub fn sorted_readings(v: &[f64]) -> Vec<f64> {
    let mut readings: Vec<f64> = v.iter().copied().filter(|x| !x.is_nan()).collect();
    readings.sort_by(|a, b| a.partial_cmp(b).unwrap());
    readings
}
```

`spread` は、並べた測定値の両端から求めることもできます。

```rust
pub fn spread(v: &[f64]) -> Option<f64> {
    let sorted = sorted_readings(v);
    Some(sorted.last()? - sorted.first()?)
}
```
