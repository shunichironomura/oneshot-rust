`Result` は `FromIterator` を実装しています。`Result<i32, E>` を返すイテレータを `Result<Vec<i32>, E>` に
`collect` すると、すべての値の `Ok` か、最初の `Err` になります。`Err` が出た時点で止まり、残りの要素は見ません。
何を作るかは返り値の型から `collect` に伝わります。`parse` は空白を読み飛ばさないので、ルールのとおりの動きに
なります。

`str::parse::<i32>` を `map` に直接渡すとコンパイルできません（E0631）。イテレータの要素は `&&str` ですが、
この関数が受け取るのは `&str` だからです。クロージャで書けば、メソッド呼び出しがレシーバを自動で参照外しします
（`map` の前に `.copied()` を挟んでも動きます）。

### 別解

普通のループでも書けます。ループの中の `?` は、最初の無効な要素のところで関数から返ります。

```rust
use std::num::ParseIntError;

pub fn parse_all(items: &[&str]) -> Result<Vec<i32>, ParseIntError> {
    let mut values = Vec::new();
    for item in items {
        values.push(item.parse::<i32>()?);
    }
    Ok(values)
}
```
