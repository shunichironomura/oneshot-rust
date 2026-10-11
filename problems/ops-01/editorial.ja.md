`impl<T>` の中では `T` について何もわからないので、`self.x + rhs.x` はコンパイルできません。
それぞれの `impl` で、本体が使う演算を**その結果の型も含めて**要求する必要があります。

- `Add` には `T: Add<Output = T>` が必要です。単なる `T: Add` では `T + T` が別の型になりうるので、
  テンプレートの `type Output = Vec2<T>` と合いません。
- `Mul<T>` には `T: Mul<Output = T> + Copy` が必要です。スカラー `k` を 2 回使うので、`Copy` がないと
  1 回目の掛け算で `k` がムーブされてしまいます。
- `Neg` には `T: Neg<Output = T>`、`AddAssign` には `T: AddAssign` が必要です。
- `dot` には `Add<Output = T>` と `Mul<Output = T>` の両方が必要です。別の `impl<T: ...> Vec2<T>` ブロックに
  分けておけば、`new` はどんな `T` でも使えるままです。

`self` と `rhs` は値で受け取るので、フィールドをムーブして取り出せます。`Add` や `Neg` に `Clone` は要りません。

### 別解

出力の型は `Vec2<T>` でなくてもかまいません。関連型 `T::Output` を使えば、`T` 同士の和が別の型になる場合も含めて、
任意の `T: Add` について実装できます。

```rust
impl<T: Add> Add for Vec2<T> {
    type Output = Vec2<T::Output>;

    fn add(self, rhs: Vec2<T>) -> Vec2<T::Output> {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}
```

境界が多くなってきたら、`where` 節に書くと読みやすくなります。
