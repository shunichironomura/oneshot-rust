数の集まりをまとめて解析する関数 `parse_all` を実装してください。要素の中には、無効なものがあるかもしれません。

```rust
use std::num::ParseIntError;

pub fn parse_all(items: &[&str]) -> Result<Vec<i32>, ParseIntError>
```

- 要素 `item` は、`item.parse::<i32>()` が成功するとき**有効**で、その値は解析した数です。要素は与えられたとおりに
  解析し、空白は取り除きません。無効な要素のエラーは、`item.parse::<i32>()` が返す `ParseIntError` です。
- `parse_all` は、すべての要素が有効なら、すべての要素の値を入力の順に並べて `Ok` で返します（空のスライスなら
  空の `Vec`）。そうでなければ、**最初の**無効な要素のエラーを `Err` で返します。

### 例

```rust
parse_all(&["7", "-3"])                  // Ok(vec![7, -3])
parse_all(&["1", "x", ""])               // Err(e)。e は "x" のエラー（不正な数字）で、
                                         // "" のエラー（空文字列）ではない
```
