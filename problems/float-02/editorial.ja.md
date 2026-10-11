`to_int_unchecked` は、浮動小数点数を何もチェックせずに整数に変換します。値が NaN や無限大ではなく、
小数部分を切り捨てたときに変換先の型に収まることが条件で、それ以外は未定義動作です。`SAFETY` コメントは
サンプルが `0.0..=1.0` に収まることを前提にしていますが、問題文ではどんな値もありえます。例の `1.5` は
レベル `383` になって `u8` に収まらず、Miri はそこで止まります。

問題文が求める動作は、普通の `as` キャストがそのまま実現しています。
Rust 1.45 以降、浮動小数点数から整数への `as` キャストは**飽和**します。変換先の型の最小値より
小さい値は最小値（`u8` なら `0`）に、最大値より大きい値は最大値（`255`）に、NaN は `0` になります。
小数部分は切り捨てられるので、先に丸めておきます。`as` キャストが UB を起こすことはないので、
`unsafe` は要りません。`to_int_unchecked` は、範囲を確認済みのコードがこのチェックを省くための
ものです。

`to_int_unchecked` を残して `.clamp(0.0, 255.0)` を足すだけでは足りません。
`f32::clamp` は入力が NaN なら NaN を返すので、NaN のサンプルではやはり UB になります
（`f32::max` と `f32::min` は違い、片方の引数が NaN ならもう片方を返します）。

### 別解

`as` が範囲外の値をどう扱うかに頼らず、比較ですべての場合を書き分ける方法です。NaN との比較は
どれも `false` になるので、NaN は別にチェックします。

```rust
pub fn to_levels(samples: &[f32]) -> Vec<u8> {
    let mut levels = Vec::with_capacity(samples.len());
    for &s in samples {
        let level = (s * 255.0).round();
        let level = if level.is_nan() || level <= 0.0 {
            0
        } else if level >= 255.0 {
            255
        } else {
            level as u8
        };
        levels.push(level);
    }
    levels
}
```

`to_int_unchecked` に渡る値がすべて範囲内なら、使い続けてもかまいません。先に NaN を処理してから
`clamp` します。

```rust
pub fn to_levels(samples: &[f32]) -> Vec<u8> {
    samples
        .iter()
        .map(|&s| {
            let level = (s * 255.0).round();
            if level.is_nan() {
                return 0;
            }
            // SAFETY: `level` is not NaN, so after clamping it is a whole number in 0.0..=255.0.
            unsafe { level.clamp(0.0, 255.0).to_int_unchecked::<u8>() }
        })
        .collect()
}
```
