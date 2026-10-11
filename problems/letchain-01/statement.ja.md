あるコマンドラインツールは、読み込んだトークンを種類ごとに仕分けます。1 つのトークンの種類を判定する
`classify` と、2 つのバイトをオーバーフローさせずに足す小さな補助関数 `sum_first_two` を実装してください。
列挙型 `Token` は用意されています（エディタに入っています）。

```rust
#[derive(Debug, PartialEq)]
pub enum Token {
    Tiny(u64),
    Small(u64),
    Large(u64),
    Hex(u32),
    Pair(String, String),
    Word(String),
}

pub fn classify(tok: &str) -> Token
pub fn sum_first_two(nums: &[u8]) -> Option<u8>
```

`classify(tok)` は、次の規則のうち最初に当てはまるものを使います。`tok` はどんな文字列でもよく、
そのまま使います（前後の空白は取り除きません）。

1. `tok.parse::<u64>()` が成功して値 `n` が得られたら、トークンは数です。`n < 10` なら `Tiny(n)`、
   `10 <= n < 1000` なら `Small(n)`、`n >= 1000` なら `Large(n)` です。先頭の `+` や 0 は受け付けます
   （`"+7"` も `"007"` も 7 です）が、`"-7"`、`" 7"`、`"7.0"` や `u64::MAX` より大きい数は受け付けません。
2. `tok` が `#` とそれに続くちょうど 6 文字からなり、その 6 文字がすべて ASCII の 16 進数字
   （`0`-`9`、`a`-`f`、`A`-`F`。大文字と小文字は混ざっていても構いません）なら、トークンは `Hex(value)` です。
   `value` は、その 6 桁を 16 進数として読んだ値です。
3. `tok` に `:` が含まれていたら、**最初の** `:` で 2 つに分けます。`:` より前の部分と後ろの部分が
   どちらも空でなければ、トークンは `Pair(before, after)` です。`:` 自体はどちらの部分にも含めません。
4. どれにも当てはまらなければ、トークンは `Word(tok)` です。空文字列もこれに当たります。

`sum_first_two(nums)` は、`nums` の最初の 2 つの要素を `a`、`b` として `Some(a + b)` を返します。
`nums` の要素が 2 つより少ないときや、`a + b` が `u8::MAX`（255）より大きいときは `None` を返します。
3 つ目以降の要素は無視します。

### 例

| 呼び出し | 結果 |
| --- | --- |
| `classify("7")` | `Tiny(7)` |
| `classify("+250")` | `Small(250)` |
| `classify("#C0ffee")` | `Hex(0xC0FFEE)` |
| `classify("#coffee")` | `Word("#coffee")`（`o` は 16 進数字ではありません） |
| `classify("a:b:c")` | `Pair("a", "b:c")` |
| `sum_first_two(&[200, 55, 9])` | `Some(255)` |
| `sum_first_two(&[200, 56])` | `None`（`256` は `u8` に収まりません） |

`Pair("a", "b:c")` は `Token::Pair("a".to_string(), "b:c".to_string())` を短く書いたものです。
