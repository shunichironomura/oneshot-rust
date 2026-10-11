下の関数はどれも、値の集まりをまとめて扱います。値の中には、無効なものや欠けているものがあるかもしれません。
4 つの関数をすべて実装してください。

```rust
use std::num::ParseIntError;

pub fn parse_all(items: &[&str]) -> Result<Vec<i32>, ParseIntError>
pub fn split_parsed<'a>(items: &[&'a str]) -> (Vec<i32>, Vec<&'a str>)
pub fn double_first(items: &[&str]) -> Result<Option<i64>, ParseIntError>
pub fn sum_all(xs: &[Option<u32>]) -> Option<u32>
```

- 要素 `item` は、`item.parse::<i32>()` が成功するとき**有効**で、その値は解析した数です。要素は与えられたとおりに
  解析し、空白は取り除きません。無効な要素のエラーは、`item.parse::<i32>()` が返す `ParseIntError` です。
- `parse_all` は、すべての要素が有効なら、すべての要素の値を入力の順に並べて `Ok` で返します（空のスライスなら
  空の `Vec`）。そうでなければ、**最初の**無効な要素のエラーを `Err` で返します。
- `split_parsed` は、有効な要素の値と、無効な要素そのものを、どちらも入力の順に並べて返します。
- `double_first` は最初の要素だけを見て、ほかの要素は有効かどうかにかかわらず無視します。空のスライスなら
  `Ok(None)` を返します。最初の要素が有効なら、その値の 2 倍を `m` として `Ok(Some(m))` を返します。そうでなければ、
  最初の要素のエラーを `Err` で返します。
- `sum_all` は、すべての要素が `Some` で、数の合計が `u32::MAX` 以下なら、合計を `Some` で返します。そうでないとき
  （`None` の要素がある、または合計が `u32::MAX` より大きい）は `None` を返します。空のスライスの合計は `0` なので、
  結果は `Some(0)` です。
- どの関数も、どんな入力に対してもパニックしてはいけません。

### 例

```rust
parse_all(&["7", "-3"])                  // Ok(vec![7, -3])
parse_all(&["1", "x", ""])               // Err(e)。e は "x" のエラー（不正な数字）で、
                                         // "" のエラー（空文字列）ではない
split_parsed(&["4", "a", "-2", "1.5"])   // (vec![4, -2], vec!["a", "1.5"])
double_first(&["21", "x"])               // Ok(Some(42))
sum_all(&[Some(1), Some(2)])             // Some(3)
sum_all(&[Some(u32::MAX), Some(1)])      // None
```
