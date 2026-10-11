`area` と `name` は必須のメソッドなので、`impl Shape for ...` ブロックごとにそれぞれの実装を書きます。
`describe` はトレイトの中に本体を持つ**デフォルトメソッド**です。`self.name()` と `self.area()` を使って 1 回だけ
書けば、ほかの場所で定義された型も含め、実装したすべての型で使えます。`format!` の `{:.2}` は浮動小数点数を
小数点以下 2 桁に丸めます。

`&dyn Shape` は `Shape` を実装した任意の型への参照で、`shape.area()` の呼び出しは実行時に正しい実装へ振り分けられます。
そのおかげで、1 つのスライスに `Rect` と `Circle` を一緒に入れられます。`f64` のイテレータを `sum()` で合計すると、
空のスライスでは `0.0` になります。

### 別解

`total_area` は普通のループでも書けます。

```rust
pub fn total_area(shapes: &[&dyn Shape]) -> f64 {
    let mut total = 0.0;
    for shape in shapes {
        total += shape.area();
    }
    total
}
```

`name` は `&'static str` を返すこともできますが、ここではシグネチャが決まっているので、`.to_string()` や
`String::from` で `String` を作ります。
