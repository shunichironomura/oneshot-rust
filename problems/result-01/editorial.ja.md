`str::parse::<i64>()` は `Result<i64, ParseIntError>` を返します。`?` 演算子は `Ok` なら中身を取り出し、`Err` なら
そのまま関数から返します。そのため最初の正しくない要素でエラーが決まり、残りはパースされません。

`parse` は空白を読み飛ばさないので、`" 2"` は `trim` してからでないと失敗します。先頭の `+` は受け付けます。

`"".split(',')` は要素を 0 個ではなく空の要素を 1 個返します。先頭のチェックがないと、空の入力がエラーになります。

### 別解

`Result` は `Sum` を実装しています。`Result<i64, E>` のイテレータを合計すると、`Ok(合計)` か、最初に現れた
`Err` になります。

```rust
use std::num::ParseIntError;

pub fn sum_csv(s: &str) -> Result<i64, ParseIntError> {
    if s.trim().is_empty() {
        return Ok(0);
    }
    s.split(',').map(|item| item.trim().parse::<i64>()).sum()
}
```
