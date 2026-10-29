`iter::successors` は、各要素を 1 つ前の要素から計算する列を作り、クロージャが `None` を
返すと終わります。`checked_mul` はかけ算がオーバーフローするときにちょうど `None` を返すので、
収まる最大の累乗の後で列が終わります。`p * base` と書くと、ジャッジのようなデバッグビルドでは
オーバーフローでパニック（"attempt to multiply with overflow"）し、リリースビルドでは黙って値が
一周します。初期値は `Some(1u64)` と書きます。`Some(1)` だと、クロージャが `checked_mul` を
呼ぶ時点で `p` の型がまだ決まっておらず、コンパイルエラーになります。浮動小数点数の対数に
逃げるのもうまくいきません。`u64::MAX as f64` はちょうど 2⁶⁴ に切り上げられるので、`2` や `16`
のような底では対数がちょうど `64` や `16` になり、収まらない 2⁶⁴ まで数えてしまいます。

### 別解

`checked_pow` と `map_while`（最初の `None` で止まる `map`）でも書けます。指数の `0..` は、
`checked_pow` が受け取る型の `u32` になります。

```rust
pub fn powers_of(base: u64) -> Vec<u64> {
    (0..).map_while(|e| base.checked_pow(e)).collect()
}
```

正確な整数の対数 `ilog` で、先に最大の指数を求める方法もあります。

```rust
pub fn powers_of(base: u64) -> Vec<u64> {
    (0..=u64::MAX.ilog(base)).map(|e| base.pow(e)).collect()
}
```
