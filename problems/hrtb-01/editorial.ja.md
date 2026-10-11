テンプレートの `'a` は `map_lines` のライフタイム引数なので、**呼び出し側**が決め、呼び出し全体より長く生きます。
ところが `f` に渡す各行は、ローカルな `String` である `normalized` を借用しており、`normalized` は関数から戻るときに破棄されます。
呼び出し側が決めた 1 つの `'a` では、この借用を表せません。

`f` に本当に求めたいのは「入力がどんなライフタイムでも、それと同じライフタイムのスライスを返す」という約束です。
省略形の `F: Fn(&str) -> &str` がちょうどこれを表していて、高階トレイト境界 `F: for<'a> Fn(&'a str) -> &'a str` の
略記になっています。`|s| s.trim()` のようなクロージャも、`first_word` のような関数もこれを満たします。

ほかにも思いつきやすいシグネチャはありますが、うまくいきません。`F: Fn(&str) -> String` では `&str` を返す
クロージャを渡せません。型引数 `R` を使った `F: Fn(&str) -> R` は `first_word` で失敗します。返り値の型が入力の
ライフタイムに依存するので、1 つの型 `R` には決まらないからです。

### 別解

高階トレイト境界を明示的に書けます。

```rust
pub fn map_lines<F>(text: &str, f: F) -> Vec<String>
where
    F: for<'a> Fn(&'a str) -> &'a str,
```

引数位置の `impl Trait` でも書けます。

```rust
pub fn map_lines(text: &str, f: impl Fn(&str) -> &str) -> Vec<String>
```
